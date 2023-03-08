#!/usr/bin/bash

docker build . -t rusty_port_scanner_ci
docker run rusty_port_scanner_ci
