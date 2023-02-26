use clap::Parser;
use dashmap::DashMap;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::sync::Arc;

use crate::cli::Cli;
use crate::port_scanner::PortScanner;

pub mod cli;
pub mod port_scanner;

const COMMON_PORTS_PATH: &str = "common_ports.csv";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let cli = Cli::parse();

    println!("Scanning on addr {}", cli.addr);

    // let mut hashmap = HashMap::new();
    let mut port_scanner = PortScanner::from(&cli);

    // sets ports to supplied ones or defaults to all
    let (start_port, end_port) = cli.ports.unwrap_or((0, 65535));

    // match function for loading the 1000 most common ports
    let mut ports: Vec<u16> = match cli.common_ports {
        true => {
            let mut v = vec![];

            let str = std::fs::read_to_string(COMMON_PORTS_PATH).expect("common ports file path");

            for i in str.split(",\n") {
                if i.parse::<u16>().is_ok() {
                    v.push(i.parse().unwrap());
                }
            }

            v
        }
        false => (start_port..=end_port).collect(),
    };

    // shuffles port numbers so firewalls blocking sequential port reads shouldn't be an issue
    ports.shuffle(&mut thread_rng());

    port_scanner.scan_ports(ports).await;

    println!(
        "{} open ports found!",
        count_open_ports(&port_scanner.port_map)
    );
}

pub fn count_open_ports(hashmap: &Arc<DashMap<u16, bool>>) -> u16 {
    hashmap.as_ref().into_iter().filter(|x| *x.value()).count() as u16
}

#[cfg(test)]
mod tests {
    use crate::cli::PORT_RANGE;

    use super::*;

    #[test]
    fn test_port_count() {
        let hashmap = Arc::new(DashMap::new());

        for i in PORT_RANGE {
            hashmap.insert(i, true);
        }

        assert_eq!(count_open_ports(&hashmap), PORT_RANGE.max().unwrap())
    }
}
