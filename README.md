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

## Caveats

Apparently something like this `1,2-5/3,4,*` is also valid according to [crontab.guru](https://crontab.guru/#1_1,2-5/3,4,*_2-3_4/5_6,7).
But this code is made with assumption that only one of `,`, `-`, `/` are used at once for each slot which colud be updated to reflect that later.
