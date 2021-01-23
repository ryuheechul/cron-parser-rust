# cron-parser

It's just a practice of attempting to parse crontab expressions with Rust

## Prerequisites

### Install Rust
https://www.rust-lang.org/tools/install

### Install Docker (optional)
https://docs.docker.com/engine/install

## With Docker

### Build
`make build`

### Test
`make test`

### Run
`make run`

[docker-compose.yaml](./docker-compose.yaml) already provides example argument
## With Cargo

### Fast run
`cargo run "*/15 0 1,15 * 1-5 /usr/bin/find"`

### Format
`cargo fmt`

### Test
`cargo test`

### Build
`cargo build [--release]`

### Run
`target/[debug|release]/cron-parser "*/15 0 1,15 * 1-5 /usr/bin/find"`

