# Dockerfile multi-stage para Railway
FROM rust:1.75-slim as builder

# Instala dependências do sistema
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Cria diretório de trabalho
WORKDIR /app

# Copia arquivos de configuração
COPY Cargo.toml ./
COPY rust-toolchain.toml ./

# Copia código fonte
COPY src ./src
COPY static ./static

# Build em release mode
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

# Instala dependências runtime
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Cria usuário não-root
RUN useradd -m -u 1000 appuser

WORKDIR /app

# Copia binário compilado
COPY --from=builder /app/target/release/registros-json /app/registros-json
COPY --from=builder /app/static /app/static

# Muda para usuário não-root
USER appuser

# Expõe porta (Railway usa variável PORT)
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=40s --retries=3 \
    CMD curl -f http://localhost:${PORT:-3000}/api/health || exit 1

# Comando de inicialização
CMD ["./registros-json"]
