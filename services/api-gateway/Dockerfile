FROM rust:1.70-slim as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Copy the Cargo files for dependency caching
COPY Cargo.toml Cargo.lock* ./

# Create a dummy main.rs to build dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -f src/main.rs

# Copy the actual source code
COPY . .

# Build the application
RUN cargo build --release

# Create a smaller runtime image
FROM debian:11-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y libssl1.1 ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/api-gateway .

# Set executable permissions
RUN chmod +x api-gateway

# Expose port
EXPOSE 8000

# Set the entrypoint
CMD ["./api-gateway"] 