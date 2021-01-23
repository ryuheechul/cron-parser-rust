FROM rust:1.43

RUN USER=root cargo new --bin cron-parser
WORKDIR ./cron-parser
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
# RUN cargo build --release
RUN rm src/*.rs

COPY . ./
RUN cargo build --release

ENTRYPOINT ["target/release/cron-parser"]
