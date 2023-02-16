use clap::Parser;
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::ops::RangeInclusive;

//TODO: add a common ports collection, can be found from awk '$2~/tcp$/' /usr/share/nmap/nmap-services | sort -r -k3 | head -n 1000 | tr -s ' ' | cut -d '/' -f1 | sed 's/\S*\s*\(\S*\).*/\1,/'
const PORT_RANGE: RangeInclusive<usize> = 1..=65535;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Addr to scan
    addr: IpAddr,

    /// Ports to scan supplied with a hyphen between them
    #[arg(value_parser = port_in_range)]
    ports: Option<(u16, u16)>,
}

fn port_in_range(s: &str) -> Result<(u16, u16), String> {
    if s.split_once('-').is_none() {
        return Err("port range missing hyphen, must be in start-end format, Ex: 1-16".into());
    }
    let (start, end) = s.split_once('-').unwrap();

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

    for i in start_port..=end_port {
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
