pub mod cli;
pub mod port_scanner;

use dashmap::DashMap;
use rust_embed::RustEmbed;
use std::sync::Arc;

#[derive(RustEmbed)]
#[folder = "./"]
#[include = "*.csv"]
///
/// A struct meant really only to grab the list of common ports that get embedded during compilation
///
pub struct Asset;

impl Asset {
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
        let file = Asset::get("common_ports.csv").unwrap();

        Ok(std::str::from_utf8(&file.data)
            .expect("common ports contents")
            .to_string())
    }
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
