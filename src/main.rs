use clap::Parser;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rusty_port_scanner::count_open_ports;

use rusty_port_scanner::cli::Cli;
use rusty_port_scanner::port_scanner::PortScanner;
use rusty_port_scanner::COMMON_PORTS_PATH;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if !cli.greppable {
        println!("Scanning on addr {}", cli.addr);
    }

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
    if !cli.sequential {
        ports.shuffle(&mut thread_rng());
    }

    smol::block_on(async { port_scanner.scan_ports(&ports, Some(&cli)).await });

    if !cli.greppable {
        println!(
            "{} open ports found!",
            count_open_ports(&port_scanner.port_map)
        );
    }
}
