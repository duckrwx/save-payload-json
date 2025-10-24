# Build stage
FROM rust:1.83-bookworm as builder

WORKDIR /app

# Instala dependências de build
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copia arquivos de dependências primeiro (cache)
COPY Cargo.toml Cargo.lock ./

# Build dummy para cachear dependências
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copia código real
COPY src ./src
COPY static ./static

# Build release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Instala dependências runtime + curl para healthcheck
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copia executável e pasta static
COPY --from=builder /app/target/release/registros-json .
COPY --from=builder /app/static ./static

# Porta correta
EXPOSE 8080
ENV PORT=8080

# Health check (agora com curl instalado)
HEALTHCHECK --interval=30s --timeout=3s --start-period=40s --retries=3 \
    CMD curl -f http://localhost:8080/api/health || exit 1

# Executa
CMD ["./registros-json"]