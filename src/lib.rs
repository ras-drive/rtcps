pub mod cli;
pub mod port_scanner;

use clap::Parser;
use dashmap::DashMap;
use rand::{seq::SliceRandom, thread_rng};
use std::sync::Arc;

use cli::Cli;
use port_scanner::PortScanner;

///
/// Returns contents of common ports file from common ports csv file that gets embedded during compilation
///
/// # Example
/// ```
/// use rtcps::get_common_ports_string;
///
/// let str = get_common_ports_string().expect("common ports file contents");
///  for port in str.split(",\n") {
///        if port.parse::<u16>().is_ok() {
///             println!("{port}");
///         }
///     }
/// ```
///
pub fn get_common_ports_string() -> Result<String, std::io::Error> {
    // let file = Asset::get("common_ports.csv").expect("common ports file");
    let file = include_bytes!("../common_ports.csv");

    Ok(std::str::from_utf8(file)
        .expect("common ports contents")
        .to_string())
}

///
/// Takes an `Arc` `DashMap` reference holding a `port_map` and returns the open ports.
///
/// # Arguments
///
/// * `hashmap` - Reference to an Arc<DashMap> holding a (u16, bool) tuple
///
/// # Example
/// ```
/// use rtcps::{count_open_ports, port_scanner::PortScanner};
///
/// let port_scanner = PortScanner::default();
/// println!("{}", count_open_ports(&port_scanner.port_map));
/// ```
#[allow(clippy::cast_possible_truncation)]
// allows truncation because the DashMap index should never go above a u16, the max index count for ports
pub fn count_open_ports(hashmap: &Arc<DashMap<u16, bool>>) -> u16 {
    hashmap.as_ref().into_iter().filter(|x| *x.value()).count() as u16
}

///
/// Function that runs the app, Taking in an optional Cli
/// or parses one while the function is running.
///
/// # Arguments
///
/// * `cli_option` - an optional Cli struct to receive args from
///
/// # Examples
/// ```
/// use rtcps::{run, cli::Cli};
/// use clap::Parser;
///
/// #[tokio::main]
/// async fn main() {
///     run(Some(Cli::parse_from(["", "127.0.0.1"]))).await;
/// }
/// ```
///
pub async fn run(cli_option: Option<Cli>) {
    let cli = if cli_option.is_some() {
        cli_option.unwrap()
    } else {
        Cli::parse()
    };

    if !cli.greppable {
        println!("Scanning on addr {}", cli.addr);
    }

    // let mut hashmap = HashMap::new();
    let mut port_scanner = PortScanner::from(&cli);

    // sets ports to supplied ones or defaults to all
    let (start_port, end_port) = cli.ports.unwrap_or((0, 65535));

    // checks for flag to use 1000 most common ports instead
    let mut ports: Vec<u16> = if cli.common_ports {
        let mut v = vec![];

        let str = get_common_ports_string().expect("common ports file contents");

        for i in str.split(",\n") {
            if i.parse::<u16>().is_ok() {
                v.push(i.parse().unwrap());
            }
        }

        v
    } else {
        (start_port..=end_port).collect()
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
