# Stage 1: Build the Rust app
FROM rust:latest as builder

WORKDIR /usr/src/fibbot
COPY . .

# Install dependencies and build the project
RUN apt-get update && apt-get install -y build-essential

RUN cargo build --release --bin fibbot \
    && mv target/release/fibbot /binary

#RUN ls -l /binary

#RUN ls -l /usr/src/fibbot/target/release/

# Stage 2: Create a distroless image for the final container
FROM gcr.io/distroless/cc
WORKDIR /app

# Copy the compiled binary from the builder stage
#COPY --from=builder /usr/src/fibbot/target/release/fibbot /usr/local/bin/fibbot
COPY --from=builder /binary /app/fibbot

#RUN ls -l /app/
#RUN echo "Contents of /app/ before ls:" && ls -l /app/

#
RUN chmod 755 /app/fibbot

ENTRYPOINT ["/app/fibbot"]



