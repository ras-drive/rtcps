use clap::Parser;
use std::net::IpAddr;
use std::ops::RangeInclusive;

pub const PORT_RANGE: RangeInclusive<u16> = 1..=65535;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// ip address to scan ex: 127.0.0.1
    pub addr: IpAddr,

    /// ports to scan supplied with a hyphen between them
    #[arg(value_parser = port_in_range)]
    pub ports: Option<(u16, u16)>,

    /// a flag to use the 1000 most common ports instead of a range
    #[arg(short, long)]
    pub common_ports: bool,

    /// when used prints each port that is open as it is found
    #[arg(short, long)]
    pub verbose: bool,
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
