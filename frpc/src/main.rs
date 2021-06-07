use frpc::{args, conf, service};
use frpc::service::service_trait::Conn;

fn main() {
    let cli_args = args::init();

    if cli_args.is_none() {
        return;
    }

    let (command, matches) = cli_args.unwrap();


    let base_conf = conf::base::load_base();

    service::run(&command, &matches, &base_conf);
}
