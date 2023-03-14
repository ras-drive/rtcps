#!/usr/bin/bash

docker build . -t rust_tcp_scanner_ci
docker run rust_tcp_scanner_ci
