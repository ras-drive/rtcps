use dashmap::DashMap;
use smol::net::TcpStream;
use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

use crate::cli::Cli;

#[derive(Clone)]
pub struct PortScanner {
    addr: IpAddr,
    pub port_map: Arc<DashMap<u16, bool>>,
}

impl PortScanner {
    pub fn new(address: IpAddr) -> Self {
        Self {
            addr: address,
            port_map: Arc::new(DashMap::new()),
        }
    }

    pub async fn check_port_open(&self, port_num: u16, cli: Option<&Cli>) -> bool {
        let socket_address = SocketAddr::new(self.addr, port_num);

        match TcpStream::connect(socket_address).await {
            Ok(_) => {
                if let Some(c) = cli {
                    if c.verbose {
                        println!("port {} open!", port_num);
                    }
                }
                true
            }
            Err(_) => false,
        }
    }

    pub async fn scan_ports(&mut self, ports: Vec<u16>, cli: Option<&Cli>) {
        for x in ports {
            if self.check_port_open(x, cli).await {
                self.port_map.insert(x, true);
            } else {
                self.port_map.insert(x, false);
            }
        }
    }
}

impl From<&Cli> for PortScanner {
    fn from(cli: &Cli) -> Self {
        Self {
            addr: cli.addr,
            port_map: Arc::new(DashMap::new()),
        }
    }
}
