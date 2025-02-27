FROM rust:alpine AS builder
WORKDIR /app
COPY . .
RUN cargo run
#RUN cargo build --release

#FROM alpine:latest
#WORKDIR /app
#FROM gcr.io/distroless/cc AS runtime
#COPY --from=builder /app/target/release/fibbot /app/fobbot
#ENTRYPOINT ["/app/fibbot"]   
