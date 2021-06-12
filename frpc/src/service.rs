use crate::{args, conf};
use std::env;
use getopts::Matches;
use crate::service::service_trait::Conn;
use tokio::{net::{TcpStream, TcpListener}, runtime::Runtime, task};
use std::error::Error;
use futures::{future, prelude::*};
use futures::{AsyncReadExt, AsyncWriteExt};
use futures::task::{Spawn, SpawnExt};
use tokio_util::compat::{Compat, TokioAsyncReadCompatExt};
use std::task::{Context, Poll, Waker};
use bytes::{BytesMut, BufMut};
use serde::__private::fmt::Write;


pub mod tcp_service;
pub mod udp_service;
pub mod service_trait;

pub fn run(command: &args::Command, sub_args: &Matches, config: &conf::base::Config) {
    dbg!(command);
    match command {
        args::Command::Start => start(sub_args, config),
        _ => panic!("wait")
    }
}

async fn login() -> Result<(), Box<dyn Error>> {
    let req_body = frp_msg::msg::Login {
        version: "0.18.0".to_string(),
        hostname: "cary".to_string(),
        os: env::consts::OS.to_string(),
        arch: env::consts::ARCH.to_string(),
        user: "".to_string(),
        privilege_key: "".to_string(),
        timestamp: 0,
        run_id: "".to_string(),
        metas: Default::default(),
        pool_count: 0,
    };


    let mut stream = TcpStream::connect("10.211.55.6:9090").await.expect("connect").compat();
    let c = yamux::Config::default();
    let conn = yamux::Connection::new(stream, c, yamux::Mode::Client);
    let mut control = conn.control();
    task::spawn(yamux::into_stream(conn).for_each(|_| future::ready(())));
    let st = control.open_stream().await?;
    let (mut r, mut w) = AsyncReadExt::split(st);

    let mut msg_buf = BytesMut::new();
    let type_byte = frp_msg::msg::get_type_byte(frp_msg::msg::MsgType::TypeLogin(&req_body));
    let resq_json = serde_json::to_string(&req_body)?;

    msg_buf.put_u8(type_byte.unwrap());
    msg_buf.put_i64(resq_json.len() as i64);
    msg_buf.put_slice((&resq_json).as_ref());

    w.write_all(&msg_buf).await?;
    let mut r_buf = [0; 1024];
    r.read(&mut r_buf).await?;
    dbg!(r_buf);
    Ok(())
}

pub fn start(sub_args: &Matches, config: &conf::base::Config) {
    let rt = Runtime::new().unwrap();
    rt.block_on(login());
    // let tcp_service = tcp_service::TcpService::new(&config.common);
    // tcp_service.start();
}