# ============================================
# STAGE 1: Build com MUSL (binário estático)
# ============================================
FROM rust:alpine AS builder

# Instala dependências de build
RUN apk add --no-cache musl-dev pkgconfig openssl-dev openssl-libs-static

WORKDIR /app

# Copia dependências primeiro (para cache)
COPY Cargo.toml Cargo.lock ./

# Build dummy para cachear dependências
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copia código real
COPY src ./src
COPY static ./static

# Build release com target estático
RUN cargo build --release

# ============================================
# STAGE 2: Runtime mínimo
# ============================================
FROM alpine:latest

# Instala apenas certificados SSL
RUN apk add --no-cache ca-certificates

WORKDIR /app

# Copia executável estático e pasta static
COPY --from=builder /app/target/release/registros-json .
COPY --from=builder /app/static ./static

# Porta
EXPOSE 8080
ENV PORT=8080

# Executa
CMD ["./registros-json"]