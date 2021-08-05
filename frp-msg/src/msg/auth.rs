use std::collections::HashMap;
use serde_derive::{Deserialize, Serialize};
use frp_msg_derive::MsgBaseTrait;

#[derive(Debug, Serialize, Deserialize, MsgBaseTrait)]
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

#[derive(Debug, Serialize, Deserialize, MsgBaseTrait)]
pub struct LoginResp {
    version: String,
    run_id: String,
    server_udp_port: isize,
    error: String,
}