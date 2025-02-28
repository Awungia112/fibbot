# Stage 1: Build the Rust app
FROM rust:latest as builder

WORKDIR /usr/src/fibbot
COPY . .

# Install dependencies and build the project
RUN apt-get update && apt-get install -y build-essential
RUN cargo build --release

# Stage 2: Create a distroless image for the final container
FROM gcr.io/distroless/cc

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/fibbot/target/release/fibbot /usr/local/bin/fibbot

ENTRYPOINT ["/usr/local/bin/fibbot"]
