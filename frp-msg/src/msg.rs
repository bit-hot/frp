use crate::msg::login::{Login, LoginResp};
pub mod login;

pub trait MsgBase {
    fn get_body(&self) -> String;
    fn get_head_byte(&self) -> u8;
}