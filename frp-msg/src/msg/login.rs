use std::collections::HashMap;
use serde_derive::{Deserialize, Serialize};
use crate::msg::{MsgBase};

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResp {
    version: String,
    run_id: String,
    server_udp_port: isize,
    error: String,
}

impl MsgBase for Login {
    fn get_body(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    fn get_head_byte(&self) -> u8 {
        crate::head::LOGIN as u8
    }
}