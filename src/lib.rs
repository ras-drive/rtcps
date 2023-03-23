pub mod cli;
pub mod port_scanner;

use chrono::Local;
use clap::Parser;
use dashmap::DashMap;
use env_logger::Builder;
use log::{Level, LevelFilter};
use rand::{seq::SliceRandom, thread_rng};
use std::io::Write;
use std::sync::Arc;

use cli::Cli;
use port_scanner::PortScanner;

#[cfg(windows)]
macro_rules! PATH_SEPARATOR {
    () => {
        r"\"
    };
}

#[cfg(unix)]
macro_rules! PATH_SEPARATOR {
    () => {
        r"/"
    };
}

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
pub fn get_common_ports_string() -> Result<String, std::str::Utf8Error> {
    let file = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        PATH_SEPARATOR!(),
        "common_ports.csv"
    ));

    Ok(std::str::from_utf8(file)?.to_string())
}

pub fn init_logger() {
    Builder::new()
        .format(|buf, record| {
            if record.level().eq(&Level::Info) {
                writeln!(buf, "{}", record.args())
            } else {
                writeln!(
                    buf,
                    "{} {}: {}",
                    record.level(),
                    //Format like you want to: <-----------------
                    Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                    record.args()
                )
            }
        })
        .filter(None, LevelFilter::Info)
        .init();
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
    init_logger();

    let cli = if cli_option.is_some() {
        cli_option.unwrap()
    } else {
        Cli::parse()
    };

    if !cli.greppable {
        log::info!("Scanning on addr {}", cli.addr);
    }

    // let mut hashmap = HashMap::new();
    let mut port_scanner = PortScanner::from(&cli);

    // sets ports to supplied ones or defaults to all
    let (start_port, end_port) = cli.get_ports();

    // checks for flag to use 1000 most common ports instead
    let mut ports: Vec<u16> = if cli.common_ports {
        let mut v = vec![];

        let str = match get_common_ports_string() {
            Ok(s) => s,
            Err(e) => {
                log::error!(
                    "error reading common ports list that gets embedded during compilation {e}"
                );
                String::new()
            }
        };

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
        log::info!(
            "{} open ports found!",
            count_open_ports(&port_scanner.port_map)
        );
    }
}
