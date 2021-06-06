use getopts::{Options, Matches};
use std::env;

#[derive(Debug)]
pub enum Command {
    Start,
    Restart,
    Reload,
    Stop,
    Status,
}

fn help(program: &str, opts: &Options) {
    print!("{}", opts.usage(&format!("Usage: {} FILE [options]", program)));
}

// init cli arguments
pub fn init() -> Option<(Command, Matches)> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt("c", "", "frpc config file", "c_file");
    opts.optflag("h", "help", "Print this help menu");

    if args.len() == 1 || args[1] == "-h" {
        help(&program, &opts);
        return None;
    }


    let command = match args[1].as_ref() {
        "start" => Command::Start,
        "restart" => Command::Restart,
        "reload" => Command::Reload,
        "stop" => Command::Stop,
        "status" => Command::Status,
        _ => panic!("Please input command"),
    };

    if args.len() > 2 {
        let matches = match opts.parse(&args[2..]) {
            Ok(m) => m,
            Err(e) => panic!("{}", e)
        };
        if matches.opt_present("h") {
            help(&program, &opts);
        }
        return Some((command, matches));
    }
    return None;
}