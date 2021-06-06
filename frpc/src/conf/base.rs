use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Common {
    server_addr: String,
    server_port: u16,
    token: String,
    user: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    common: Common
}

pub fn load_base() -> Config {
    let base_conf_content = include_str!("../../Conf.toml");
    let conf: Config = toml::from_str(base_conf_content).unwrap();
    conf
}