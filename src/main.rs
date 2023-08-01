use std::error::Error;
use std::time::Duration;
use std::io;

use clap::Parser;
use reqwest::blocking::Client;
use serde_json::Value;

use dialoguer::{theme::ColorfulTheme, Select};
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Parser)]
#[command(name = "Make a Request to Wolfram Alpha API")]
#[command(about = "Wolfram Alpha API")]
struct ReqArgs {
    /// Input to the Wolfram Alpha API
    #[arg()]
    input: String,
    /// Use this to enable interactive mode; `exit` to exit interactive mode
    #[arg(short, long, default_value_t = false)]
    interactive: bool,
    /// Extra params to the Wolfram Alpha API i.e. Step-by-step solution
    #[arg(long, default_value = "Step-by-step solution")]
    podstate: String,
    /// Total timeout
    #[arg(short, long, default_value_t = 30)]
    totaltimeout: u8,
    /// Pod timeout. Individual computation block timeout
    #[arg(short, long, default_value_t = 30)]
    podtimeout: u8,
    /// Format timeout
    #[arg(long, default_value_t = 30)]
    formattimeout: u8,
    /// Parse timeout
    #[arg(long, default_value_t = 30)]
    parsetimeout: u8,
    /// Scan timeout
    #[arg(long, default_value_t = 30)]
    scantimeout: u8,
    /// Specify the appid used by api to determine source of the request.
    #[arg(long, default_value = "H9V325-HTALUWHKGK")]
    appid: String,
    /// Does Wolfram need to reinterpret the input.
    #[arg(long, default_value_t = true)]
    reinterpret: bool,
}

fn get_response(req_args: &ReqArgs) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            // For more spinners check out the cli-spinners project:
            // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
            .tick_strings(&[
                "[    ]", "[=   ]", "[==  ]", "[=== ]", "[ ===]", "[  ==]", "[   =]", "[    ]",
                "[   =]", "[  ==]", "[ ===]", "[====]", "[=== ]", "[==  ]", "[=   ]",
            ]),
    );
    pb.set_message("\x1b[33mRetrieving response...\x1b[0m");
    let response = client
        .get("https://api.wolframalpha.com/v2/query")
        .query(&[("output", "json".to_string())])
        .query(&[("input", req_args.input.to_string())])
        .query(&[("podstate", req_args.podstate.to_string())])
        .query(&[("totaltimeout", req_args.totaltimeout.to_string())])
        .query(&[("podtimeout", req_args.podtimeout.to_string())])
        .query(&[("formattimeout", req_args.formattimeout.to_string())])
        .query(&[("parsetimeout", req_args.parsetimeout.to_string())])
        .query(&[("scantimeout", req_args.scantimeout.to_string())])
        .query(&[("appid", req_args.appid.to_string())])
        .query(&[("reinterpret", req_args.reinterpret.to_string())])
        .send()?
        .text()?;
    pb.finish_with_message("\x1b[32mResponse retrieved:\x1b[0m");
    Ok(response)
}

fn response_to_json(response: String) -> Result<Value, Box<dyn Error>> {
    let v: Value = serde_json::from_str(&response)?;
    Ok(v)
}

fn json_to_formatted_string(v: Value) -> Result<String, Box<dyn Error>> {
    let num_pods = v["queryresult"]["numpods"].as_u64().unwrap_or(0);
    let mut pods_amount = 0..num_pods;
    let selections = &["yes", "no"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Show full response?")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();
    let mut formatted_string = String::new();
    if selections[selection] == "no" {
        pods_amount = 0..2;
    }
    for i in pods_amount {
        let pod = &v["queryresult"]["pods"][i as usize];
        let title = pod["title"].as_str().unwrap_or("No title");
        formatted_string.push_str(&format!(
            "{}{}{}\n",
            "\x1b[33m",
            title.to_string(),
            "\x1b[0m"
        ));
        let num_subpods = pod["numsubpods"].as_u64().unwrap_or(0);
        for j in 0..num_subpods {
            let subpod = &pod["subpods"][j as usize];
            let plaintext = subpod["plaintext"].as_str().unwrap_or("No plaintext");
            formatted_string.push_str(&format!("{}\n", plaintext.to_string(),));
        }
    }
    Ok(formatted_string)
}

fn match_to_output(req_args: &ReqArgs) -> Result<String, Box<dyn Error>> {
    let response_string = match get_response(&req_args) {
        Ok(response) => response,
        Err(e) => {
            println!("{:?}", e);
            return Err(e);
        }
    };

    let v = response_to_json(response_string)?;
    let formatted_string = json_to_formatted_string(v)?;
    Ok(formatted_string)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut req_args = ReqArgs::parse();

    if (&req_args).interactive {
        println!("{}", match_to_output(&req_args)?);
        loop {
            let mut input = String::new();
            println!("{}Enter your input: {}", "\x1b[32m", "\x1b[0m");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            if input.trim().to_lowercase() == "exit" {
                return Ok(());
            } else {
                req_args.input = input;
                println!("{}", match_to_output(&req_args)?);
            }
        }
    }

    println!("{}", match_to_output(&req_args)?);

    Ok(())
}
