FROM rust:latest AS build
WORKDIR /app
COPY . .
RUN cargo build --release

FROM Ubuntu:latest
FROM gcr.io/distroless/cc AS runtime

COPY --from=build /app/target/release/fibbot /app/fibbot
ENTRYPOINT ["/app/fibbot"]
