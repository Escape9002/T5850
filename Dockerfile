# -------- Build Stage --------
FROM rust:slim AS build

# Set MUSL target for static linking
RUN rustup target add x86_64-unknown-linux-musl && \
    apt-get update && \
    apt-get install -y musl-tools musl-dev && \
    update-ca-certificates

# Compile dependencies and save them in a layer
WORKDIR /app
RUN mkdir src && echo 'fn main(){println!("This is the cached layer talking to you!");}' > src/main.rs
COPY Cargo.toml .
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build --release --target x86_64-unknown-linux-musl

# Build release binary
COPY src src
# Update timestamp of main.rs to force rebuild
RUN touch src/main.rs
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build --release --target x86_64-unknown-linux-musl

# -------- Final Runtime Stage --------
FROM debian:trixie AS runtime
   
# Copy binary from build stage
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/gpn_bot /app/gpn_bot
    
# Copy environment file
# COPY .env /app/.env
    
# Switch to unprivileged user
    
WORKDIR /app
ENTRYPOINT ["./gpn_bot"]