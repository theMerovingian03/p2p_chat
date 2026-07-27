# IMP TODO: Currently copies everything from workspace, configure cargo-chef to properly handle package compiling and shipping.

# Containerize and deploy the axum service
# uses rust 1.96.x for debian dist
FROM rust:1.96-bookworm AS builder 

# Container working directory
WORKDIR /app

# # Backend requires "server" and "shared" packages
# COPY Cargo.toml Cargo.lock ./
# COPY server/Cargo.toml server/Cargo.toml
# COPY shared/Cargo.toml shared/Cargo.toml

# # Create dummy sources to build dependencies (this will cache the dependencie)
# RUN mkdir -p server/src shared/src && \ 
#     # write temporary code in server/src/main.rs
#     echo "fn main() {}" > server/src/main.rs && \
#     # write temporary code in shared/src/lib.rs
#     echo "" > shared/src/lib.rs

# RUN cargo build --release -p server

# # Remove dummy sources
# RUN rm -rf server/src shared/src

# Copy real source
COPY . .

RUN cargo build --release -p server

# --------- RUN
FROM debian:bookworm-slim

# Update packages
RUN apt-get update && \
    # Install CA certificates
    apt-get install -y ca-certificates && \
    # Delete download package indexes
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/server /usr/local/bin/server

EXPOSE 8080

CMD ["server"]