use frpc::args;
use std::path::Path;

fn main() {
    let cli_args = args::init();

    if cli_args.is_none() {
        return;
    }

    let (command, matches) = cli_args.unwrap();
    // 获取配置文件
    let cnf_file = matches.opt_get::<String>("c").unwrap().unwrap();
    let cnf = frp_config::client::Config::new(Path::new(&cnf_file));
    dbg!(cnf_file);
    dbg!(cnf)
}
