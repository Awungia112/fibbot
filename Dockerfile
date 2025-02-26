FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine
COPY --from=builder /app/target/release/fibbot /usr/local/bin/fibbot
ENTRYPOINT ["fibbot"]
