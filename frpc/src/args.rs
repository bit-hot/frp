use std::env;
use getopts::{Options, Matches};
use std::error::Error;

#[derive(Debug)]
pub enum Command {
    Start,
    Restart,
    Reload,
    Stop,
    Status,
}

// init cli arguments
pub fn init() -> Result<(Command, Matches), &'static str> {
    let args: Vec<String> = env::args().collect();
    let command = match args[1].as_ref() {
        "start" => Command::Start,
        "restart" => Command::Restart,
        "reload" => Command::Reload,
        "stop" => Command::Stop,
        "status" => Command::Status,
        _ => panic!("Please input command"),
    };

    let program = args[0].clone();
    if args.len() > 2 {
        let mut opts = Options::new();
        opts.optopt("c", "", "frpc config file", "c_file");
        opts.optflag("h", "help", "Print this help menu");
        let matches = match opts.parse(&args[2..]) {
            Ok(m) => m,
            Err(e) => panic!(e.to_string())
        };
        if matches.opt_present("h") {
            print!("{}", opts.usage(&format!("Usage: {} FILE [options]", program)));
        }
        return Ok((command, matches));
    }
    return Err("The Command arguments error.");
}