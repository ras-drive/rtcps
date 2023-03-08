use clap::Parser;
use std::net::IpAddr;
use std::ops::RangeInclusive;

pub const PORT_RANGE: RangeInclusive<u16> = 1..=65535;

/// A Clap parser struct that really only *needs* an IP address
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// ip address to scan ex: 127.0.0.1
    pub addr: IpAddr,

    /// ports to scan supplied with a hyphen between them
    #[arg(value_parser = port_in_range, conflicts_with("common_ports"))]
    pub ports: Option<(u16, u16)>,

    /// a flag to use the 1000 most common ports instead of a range
    #[arg(short, long, conflicts_with("ports"))]
    pub common_ports: bool,

    /// when used prints each port that is open as it is found
    #[arg(short, long, conflicts_with("greppable"))]
    pub verbose: bool,

    /// scans ports sequentially without randomizing list
    #[arg(short, long)]
    pub sequential: bool,

    /// outputs only port numbers for greppable output
    #[arg(short, long, conflicts_with("verbose"))]
    pub greppable: bool,
}

///
/// A Clap parser function for returning port tuples from hyphen separated strings
///
/// # Arguments
///
/// * `s` - string slice to split
///
/// # Example Inputs
///
/// * `1-65535` - scans all ports
/// * `1-1000` - scans the first 1000 ports
///

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
