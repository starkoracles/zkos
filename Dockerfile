FROM rust:latest
COPY ./ ./

WORKDIR ./recursive
RUN cargo build --release

ENTRYPOINT ["./target/release/zkprunner"]
