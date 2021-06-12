use crate::service::service_trait::Conn;
use crate::conf;
use std::net::{ToSocketAddrs, TcpListener, SocketAddr};
use std::io::Read;

// Init tcp service struct
pub struct TcpService<'a> {
    config: &'a conf::base::Common,
}

impl<'a> TcpService<'a> {
    pub fn new(config: &'a conf::base::Common) -> Self {
        TcpService {
            config
        }
    }
}

impl<'a> Conn for TcpService<'a> {
    fn start(&self) {
        let addr = SocketAddr::from((self.config.server_addr, self.config.server_port));
        let listener = TcpListener::bind(&addr).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(mut raw) => {
                    let mut buf = vec![0;1024];
                    raw.read(&mut buf);
                    let request = String::from_utf8_lossy(&buf);
                    print!("{}", request);
                },
                Err(e) => panic!("{}", e),
            }
        }
    }
}