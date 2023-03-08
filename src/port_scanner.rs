use dashmap::DashMap;
use smol::net::TcpStream;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

use crate::cli::Cli;

#[derive(Clone)]
pub struct PortScanner {
    /// IP addr to eventually scan
    addr: IpAddr,
    /// Port map to store port data
    pub port_map: Arc<DashMap<u16, bool>>,
}

impl PortScanner {
    ///
    /// returns a new Port Scanner bound to an address.
    ///
    /// # Arguments
    ///
    /// * `address` - An IpAddr to scan
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_port_scanner::port_scanner::PortScanner;
    /// use std::net::{IpAddr, Ipv4Addr};
    ///
    /// let port_scanner = PortScanner::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    /// ```
    ///
    pub fn new(address: IpAddr) -> Self {
        Self {
            addr: address,
            port_map: Arc::new(DashMap::new()),
        }
    }

    ///
    /// Really is only called internally except for in a unit test
    /// returns true if the port number supplied is open when checked.
    /// Optionally takes a Cli in order to test for flags.
    ///
    /// # Arguments
    ///
    /// * `port_num` - Port to scan
    /// * `cli` - Optional Cli arg to test for verbose flag
    ///
    /// # Example
    ///
    /// Example code that checks if a Postgresql server is running
    /// (Postgres servers are usually bound to port 5432)
    ///
    /// ```
    /// use rusty_port_scanner::port_scanner::PortScanner;
    ///
    /// let port_scanner = PortScanner::default();
    /// let postgres_is_open = port_scanner.check_port_open(&5432, None);
    /// ```
    ///
    pub async fn check_port_open(&self, port_num: &u16, cli: Option<&Cli>) -> bool {
        let socket_address = SocketAddr::new(self.addr, *port_num);

        match TcpStream::connect(socket_address).await {
            Ok(_) => {
                if let Some(c) = cli {
                    if c.verbose {
                        println!("port {} open!", port_num);
                    }

                    if c.greppable {
                        println!("{}", port_num);
                    }
                }
                true
            }
            Err(_) => false,
        }
    }

    ///
    /// Performs a port scan and mutates the structs inner port_map
    ///
    /// # Arguments
    ///
    /// * `ports` - a reference to a vec holding a list of port numbers to scan
    /// * `cli` - an optional cli arg for passing scan flags
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_port_scanner::port_scanner::PortScanner;
    ///
    /// let ports: Vec<u16> = (0..=65535).collect();
    /// let mut port_scanner = PortScanner::default();
    /// port_scanner.scan_ports(&ports, None);
    /// ```
    ///
    pub async fn scan_ports(&mut self, ports: &Vec<u16>, cli: Option<&Cli>) {
        for x in ports {
            if self.check_port_open(x, cli).await {
                self.port_map.insert(*x, true);
            } else {
                self.port_map.insert(*x, false);
            }
        }
    }
}

impl Default for PortScanner {
    ///
    /// Returns a Port Scanner bound to localhost
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_port_scanner::port_scanner::PortScanner;
    ///
    /// let port_scanner = PortScanner::default();
    /// ```
    ///
    fn default() -> Self {
        Self {
            addr: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port_map: Default::default(),
        }
    }
}

impl From<&Cli> for PortScanner {
    ///
    /// Returns a Port Scanner that inherits fields from a Cli struct
    ///
    /// # Arguments
    ///
    /// * `cli` - reference to a Cli struct
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_port_scanner::{port_scanner::PortScanner, cli::Cli};
    /// use clap::Parser;
    ///
    /// let cli = Cli::parse_from(["", "127.0.0.1"]);
    /// let port_scanner = PortScanner::from(&cli);
    /// ```
    ///
    fn from(cli: &Cli) -> Self {
        Self {
            addr: cli.addr,
            port_map: Arc::new(DashMap::new()),
        }
    }
}
