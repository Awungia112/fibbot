FROM rust:latest AS build
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine
FROM gcr.io/distroless/cc AS runtime

COPY --from=build /app/target/release/fibbot /usr/local/bin/fibbot
ENTRYPOINT ["fibbot"]
