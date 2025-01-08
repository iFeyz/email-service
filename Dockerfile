FROM rust:latest as builder

# Installation des dépendances nécessaires
RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libssl-dev \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/* && \
    rustup update

WORKDIR /usr/src/app

# Copie de tout le code source
COPY . .

# Build de l'application
RUN cargo build --release

# Image finale
FROM debian:bookworm-slim

# Installation des bibliothèques nécessaires
RUN apt-get update && \
    apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copie du binaire depuis le stage de build
COPY --from=builder /usr/src/app/target/release/email-service /app/

# Exposition des ports (HTTP, WebSocket, gRPC)
EXPOSE 3030 3031 3032

ENV RUST_LOG=info

CMD ["./email-service"]