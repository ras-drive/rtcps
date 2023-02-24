use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr},
};
use tokio::net::TcpStream;

use crate::cli::Cli;

pub struct PortScanner {
    addr: IpAddr,
    pub port_map: HashMap<u16, bool>,
}

impl PortScanner {
    pub fn new(address: IpAddr) -> Self {
        Self {
            addr: address,
            port_map: HashMap::new(),
        }
    }

    pub async fn check_port_open(&self, port_num: u16) -> bool {
        let socket_address = SocketAddr::new(self.addr, port_num);

        match tokio::spawn(async move { TcpStream::connect(socket_address).await })
            .await
            .unwrap()
        {
            Ok(_) => {
                println!("port {} open!", port_num);
                true
            }
            Err(_) => false,
        }
    }

    pub async fn scan_ports(&mut self, ports: Vec<u16>) {
        for x in ports {
            if self.check_port_open(x).await {
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
            port_map: HashMap::new(),
        }
    }
}
