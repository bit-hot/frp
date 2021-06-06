use serde_derive::Deserialize;
use std::net::IpAddr;

#[derive(Deserialize, Debug)]
pub struct Common {
    pub server_addr: IpAddr,
    pub server_port: u16,
    pub token: String,
    pub user: String,
    pub admin_addr: IpAddr,
    pub admin_port: u16,
    pub admin_user: String,
    pub admin_pwd: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub common: Common,
}

pub fn load_base() -> Config {
    let base_conf_content = include_str!("../../Conf.toml");
    let conf: Config = toml::from_str(base_conf_content).unwrap();
    conf
}