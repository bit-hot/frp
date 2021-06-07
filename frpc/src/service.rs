use crate::{args, conf};
use getopts::Matches;
use crate::service::service_trait::Conn;

pub mod tcp_service;
pub mod udp_service;
pub mod service_trait;

pub fn run(command: &args::Command, sub_args: &Matches, config: &conf::base::Config) {
    dbg!(command);
    match command {
        args::Command::Start => start(sub_args, config),
        _ => panic!("wait")
    }
}

pub fn start(sub_args: &Matches, config: &conf::base::Config) {
    let tcp_service = tcp_service::TcpService::new(&config.common);
    tcp_service.start();
}