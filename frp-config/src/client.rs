use std::net::IpAddr;
use std::path::Path;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Common {
    /// connection server address
    server_addr: IpAddr,
    /// connection server port
    server_port: u16,
    /// the file path for log output
    log_file: String,
    /// the log level such as: warn, info, debug, trace
    log_level: String,
    log_max_days: u16,
    disable_log_color: bool,
    authenticate_heartbeats: bool,
    authenticate_new_work_conns: bool,
    token: String,
    oidc_client_id: String,
    oidc_client_secret: String,
    oidc_audience: String,
    oidc_token_endpoint_url: String,
    admin_addr: IpAddr,
    admin_port: u16,
    admin_user: String,
    admin_pwd: String,
    assets_dir: String,
    pool_count: u8,
    tcp_mux: bool,
    user: String,
    login_fail_exit: bool,
    protocol: String,
    tls_enable: bool,
    tls_cert_file: Option<String>,
    tls_key_file: Option<String>,
    tls_trusted_ca_file: Option<String>,
    tls_server_name: Option<String>,
    dns_server: Option<IpAddr>,
    start: Option<String>,
    heartbeat_interval: u8,
    heartbeat_timeout: u8,
    metas: Option<Vec<String>>,
    udp_packet_size: u16,
    include_conf_files: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    common: Common,
}


impl Config {
    pub fn new(cnf_file: &Path) -> Config {
        let def_str = include_str!("../frpc_full.ini");
        let mut def_cnf = config::Config::default();
        def_cnf
            .merge(config::File::from_str(def_str, config::FileFormat::Ini)).unwrap()
            .merge(config::File::from(cnf_file)).unwrap();

        Config {
            common: def_cnf.get::<Common>("common").unwrap()
        }
    }
}