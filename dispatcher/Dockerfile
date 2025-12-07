# Build stage
FROM rust:1.91 AS builder

WORKDIR /app

# Copy all files and build
COPY dispatcher/Cargo.toml dispatcher/Cargo.lock* ./
COPY dispatcher/src ./src
COPY dispatcher/openapi.yaml ./

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/dispatcher /app/dispatcher
COPY --from=builder /app/openapi.yaml /app/openapi.yaml

ENV RUST_BACKTRACE=1
ENV PORT=8080

EXPOSE 8080

CMD ["./dispatcher"]

