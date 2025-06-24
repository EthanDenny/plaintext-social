# Build stage
FROM rust:latest as builder

WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./
COPY migration/Cargo.toml ./migration/

# Copy source code
COPY src ./src
COPY migration/src ./migration/src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/plaintext-social .

# Copy static files and configuration
COPY static ./static
COPY templates ./templates

# Expose the port
EXPOSE 9999

# Run the application
CMD ["./plaintext-social"]
