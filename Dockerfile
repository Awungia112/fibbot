FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine
FROM gcr.io/distroless/cc AS runtime
COPY --from=builder /app/target/release/fibbot /app
ENTRYPOINT ["/app/fibbot"]
