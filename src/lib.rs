pub mod cli;
pub mod port_scanner;

use dashmap::DashMap;
use std::sync::Arc;

/// project path to common ports csv file
pub const COMMON_PORTS_PATH: &str = "common_ports.csv";

///
/// Takes an `Arc` `DashMap` reference holding a `port_map` and returns the open ports.
///
/// # Example
/// ```
/// use rusty_port_scanner::{count_open_ports, port_scanner::PortScanner};
///
/// let port_scanner = PortScanner::default();
/// println!("{}", count_open_ports(&port_scanner.port_map));
/// ```
#[allow(clippy::cast_possible_truncation)]
// allows truncation because the DashMap index should never go above a u16, the max index count for ports
pub fn count_open_ports(hashmap: &Arc<DashMap<u16, bool>>) -> u16 {
    hashmap.as_ref().into_iter().filter(|x| *x.value()).count() as u16
}
