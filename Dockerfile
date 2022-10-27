FROM rust:latest
COPY ./ ./

WORKDIR ./recursive
RUN cargo build --release

ARG FRI_QUERIES=20

CMD ["./target/release/zkprunner -f $FRI_QUERIES"]
