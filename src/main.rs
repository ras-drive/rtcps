use clap::Parser;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::ops::RangeInclusive;

const PORT_RANGE: RangeInclusive<u16> = 1..=65535;
const COMMON_PORTS_PATH: &str = "common_ports.csv";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Addr to scan
    addr: IpAddr,

    /// Ports to scan supplied with a hyphen between them
    #[arg(value_parser = port_in_range)]
    ports: Option<(u16, u16)>,

    /// a flag to use the 1000 most common ports instead of a range
    #[arg(short, long)]
    common_ports: bool,
}

fn port_in_range(s: &str) -> Result<(u16, u16), String> {
    if s.split_once('-').is_none() {
        return Err("port range missing hyphen, must be in start-end format, Ex: 1-16".into());
    }
    let (start, end) = s.split_once('-').unwrap();

    if start > end {
        return Err("ending port number range should be higher than the starting number".into());
    }

    if PORT_RANGE.contains(&start.parse().unwrap()) && PORT_RANGE.contains(&end.parse().unwrap()) {
        Ok((start.parse().unwrap(), end.parse().unwrap()))
    } else {
        Err(format!(
            "port not in range {}-{}",
            PORT_RANGE.start(),
            PORT_RANGE.end()
        ))
    }
}

fn main() {
    let cli = Cli::parse();

    println!("Scanning on addr {}", cli.addr);

    let mut hashmap = HashMap::new();

    let (start_port, end_port) = cli.ports.unwrap_or((0, 65535));

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

    for i in ports {
        if check_port_open(cli.addr, i) {
            hashmap.insert(i, true);
        } else {
            hashmap.insert(i, false);
        }
    }

    println!("{} open ports found!", count_open_ports(hashmap));
}

pub fn check_port_open(addr: IpAddr, port_num: u16) -> bool {
    let socket_address = SocketAddr::new(addr, port_num);

    match TcpStream::connect(socket_address) {
        Ok(_) => {
            println!("port {} open!", port_num);
            true
        }
        Err(_) => false,
    }
}

pub fn count_open_ports(hashmap: HashMap<u16, bool>) -> u16 {
    let mut open_ports: u16 = 0;

    hashmap.iter().for_each(|(_i, p)| {
        if *p {
            open_ports += 1;
        }
    });

    open_ports
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_count() {
        let mut hashmap: HashMap<u16, bool> = HashMap::new();

        for i in PORT_RANGE {
            hashmap.insert(i, true);
        }

        assert_eq!(count_open_ports(hashmap), PORT_RANGE.max().unwrap())
    }
}
