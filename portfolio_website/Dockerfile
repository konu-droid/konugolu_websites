# syntax=docker/dockerfile:1

# ---------------------------------------------------------------------------
# Stage 1: Build the WebAssembly frontend and the Axum backend with Rust.
# ---------------------------------------------------------------------------
FROM rust:trixie AS builder

# WebAssembly target for the Yew frontend and the trunk bundler.
RUN rustup target add wasm32-unknown-unknown \
    && cargo install --locked trunk

WORKDIR /app

# Copy the whole workspace and build both crates.
# (The frontend must be built first so its dist/ exists for the runtime image.)
COPY . .

RUN cd frontend && trunk build --release
RUN cargo build --release --package backend

# ---------------------------------------------------------------------------
# Stage 2: Minimal runtime image.
# ---------------------------------------------------------------------------
FROM debian:trixie-slim

# ca-certificates is the only runtime dependency for the static file server.
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app/backend

# Compiled server binary and the bundled frontend assets.
# The backend serves "../frontend/dist" relative to its working directory.
COPY --from=builder /app/target/release/backend ./backend
COPY --from=builder /app/frontend/dist /app/frontend/dist

# Cloud Run sends traffic to $PORT (defaults to 8080); the backend honours it.
ENV PORT=8080
EXPOSE 8080

CMD ["./backend"]
