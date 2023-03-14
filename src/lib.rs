pub mod cli;
pub mod port_scanner;

use dashmap::DashMap;
use expanduser::expanduser;
use std::{path::Path, sync::Arc};

const COMMON_PORTS_PATH: &str = "~/.local/share/rtcps/common_ports.csv";

///
/// Returns contents of common ports file from either installed location or current directory if the program isn't installed
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
    let mut path = expanduser(COMMON_PORTS_PATH)?;

    if Path::new("common_ports.csv").exists() || !path.exists() {
        if !Path::new("common_ports.csv").exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "common_ports.csv file expected",
            ));
        }

        path = Path::new("common_ports.csv").to_path_buf();
    }

    std::fs::read_to_string(path)
}

///
/// Takes an `Arc` `DashMap` reference holding a `port_map` and returns the open ports.
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
