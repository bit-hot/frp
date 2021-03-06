/// fpr之间通过yamux发送消息类型时候的定义, 按照的字节协议, 第一个字节为消息类型
pub const LOGIN: char = 'o';
pub const LOGIN_RESP: char = '1';
pub const NEW_PROXY: char = 'p';
pub const NEW_PROXY_RESP: char = '2';
pub const CLOSE_PROXY: char = 'c';
pub const NEW_WORK_CONN: char = 'w';
pub const NEW_WORK_CONN_RESP: char = 'r';
pub const START_WORK_CONN: char = 's';
pub const NEW_VISITOR_CONN: char = 'v';
pub const NEW_VISITOR_CONN_RESP: char = '3';
pub const PING: char = 'h';
pub const PONG: char = '4';
pub const UDP_PACKET: char = 'u';
pub const NAT_HOLE_VISITOR: char = 'i';
pub const NAT_HOLE_CLIENT: char = 'n';
pub const NAT_HOLE_RESP: char = 'm';
pub const NAT_HOLE_CLIENT_DETECT_OK: char = 'd';
pub const NAT_HOLE_SID: char = '5';

pub use frp_derive::*;

pub trait FrpMsg {
    fn get_body(&self) -> String;
    fn get_head_byte(&self) -> u8;
}
