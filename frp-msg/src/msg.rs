use std::collections::HashMap;
use serde_derive::{Deserialize, Serialize};

// fpr之间通过yamux发送消息类型时候的定义, 按照的字节协议, 第一个字节为消息类型
pub const TYPE_LOGIN: char = 'o';
pub const TYPE_LOGIN_RESP: char = '1';
pub const TYPE_NEW_PROXY: char = 'p';
pub const TYPE_NEW_PROXY_RESP: char = '2';
pub const TYPE_CLOSE_PROXY: char = 'c';
pub const TYPE_NEW_WORK_CONN: char = 'w';
pub const TYPE_REQ_WORK_CONN: char = 'r';
pub const TYPE_START_WORK_CONN: char = 's';
pub const TYPE_NEW_VISITOR_CONN: char = 'v';
pub const TYPE_NEW_VISITOR_CONN_RESP: char = '3';
pub const TYPE_PING: char = 'h';
pub const TYPE_PONG: char = '4';
pub const TYPE_UDP_PACKET: char = 'u';
pub const TYPE_NAT_HOLE_VISITOR: char = 'i';
pub const TYPE_NAT_HOLE_CLIENT: char = 'n';
pub const TYPE_NAT_RESP: char = 'm';
pub const TYPE_NAT_HOLE_CLIENT_DETECT_OK: char = 'd';
pub const TYPE_NAT_HOLE_SID: char = '5';

pub enum MsgType<'a> {
    TypeLogin(&'a Login),
    TypeLoginResp(LoginResp),
}

// 通过MsgType获取发送的消息类型
pub fn get_type_byte(msg_type: MsgType) -> Option<u8> {
    let cur_type = match msg_type {
        MsgType::TypeLogin(_) => TYPE_LOGIN,
        MsgType::TypeLoginResp(_) => TYPE_LOGIN_RESP
    };
    Some(cur_type as u8)
}

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

pub struct LoginResp {
    version: String,
    run_id: String,
    server_udp_port: isize,
    error: String,
}
