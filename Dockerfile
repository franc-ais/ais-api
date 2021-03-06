FROM rust:1.22.1

RUN mkdir /app
COPY ./Cargo.toml /app/Cargo.toml
COPY ./src /app/src
WORKDIR /app
RUN cargo build --release

CMD cargo run --release -- -p $PORT
