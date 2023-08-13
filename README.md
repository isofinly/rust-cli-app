# Rust cli app

Usage: cli-app [OPTIONS] `<INPUT>`

## Arguments:

  `<INPUT>`  Input to the Wolfram Alpha API

## Options:

`-i, --interactive`                    Use this to enable interactive mode; `exit` to exit interactive mode
  
`--podstate <PODSTATE>`            Extra params to the Wolfram Alpha API i.e. Step-by-step solution [default: "Step-by-step solution"]

`-t, --totaltimeout <TOTALTIMEOUT>`    Total timeout [default: 30]

`-p, --podtimeout <PODTIMEOUT>`        Pod timeout. Individual computation block timeout [default: 30]

`--formattimeout <FORMATTIMEOUT>`  Format timeout [default: 30]
  
`--parsetimeout <PARSETIMEOUT>`    Parse timeout [default: 30]

`--scantimeout <SCANTIMEOUT>`      Scan timeout [default: 30]

`--appid <APPID>`                  Specify the appid used by api to determine source of the request [default: H9V325-HTALUWHKGK]
      
`--reinterpret`                    Does Wolfram need to reinterpret the input
      
`-h, --help`                           Print help
