use std::collections::HashMap;
use serde_derive::{Deserialize, Serialize};
use frp_trait::msg::MsgBase;

#[derive(Debug, Serialize, Deserialize, MsgBase)]
pub struct Login {
    pub version: String,
    pub hostname: String,
    pub os: String,
    pub arch: String,
    pub user: String,
    pub privilege_key: String,
    pub timestamp: i64,
    pub run_id: String,
    pub metas: HashMap<String, String>,
    pub pool_count: isize,
}

#[derive(Debug, Serialize, Deserialize, MsgBase)]
pub struct LoginResp {
    pub version: String,
    pub run_id: String,
    pub server_udp_port: isize,
    pub error: String,
}