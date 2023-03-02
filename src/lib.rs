pub mod cli;
pub mod port_scanner;

use dashmap::DashMap;
use std::sync::Arc;

pub const COMMON_PORTS_PATH: &str = "common_ports.csv";

pub fn count_open_ports(hashmap: &Arc<DashMap<u16, bool>>) -> u16 {
    hashmap.as_ref().into_iter().filter(|x| *x.value()).count() as u16
}
