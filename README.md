# What is this app?

This is an attempt at a port scanner written in Rust as a hobby project.
I thought it would be a fun little tool to make while also brushing up on
writing CLI apps now that I have more experience programming.

The common ports collection was acquired by running the following command

```bash
awk '$2~/tcp$/' /usr/share/nmap/nmap-services | sort -r -k3 | head -n 1000 | tr -s ' ' | cut -d '/' -f1 | sed 's/\S*\s*\(\S*\).*/\1,/'
```

## How to install the app

To install this app with cargo installed just run

```bash
cargo install rtcps
```

If you are on a Debian based system and want to install the binary
through your package manager you can download the provided .deb file

If you would like to build a binary from source you can run ```release.py```

| Available Binary Releases |
|---------------------------|
|           Debian          |

## How to use the app

You can build the app with Cargo installed by running the following
command in the project directory.

```bash
cargo build --release
```

Then it can be started by running

```bash
cargo run -- [ip_address_to_scan]
```

The supplied IP address can be either Ipv4 or Ipv6

By default if only an IP address was supplied it will attempt to scan
all of the ports. You can supply a range of ports to scan like so.

```bash
cargo run -- 127.0.0.1 1-65535
```

If you would only like to scan the top 1000 most common ports you can run

```bash
cargo run -- 127.0.0.1 -c
```

## Benchmarking

If you wanna run the benchmark it is backed by Criterion.rs, all you have
to do is run

```bash
cargo bench
```

If you want to generate a flamegraph run

```bash
cargo bench --bench all_port_bench -- --profile-time=5
```

### Licensing

This app is Licensed under the MIT/Apache 2.0 License
