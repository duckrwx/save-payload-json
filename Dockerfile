# Build stage
FROM rust:1.75-slim as builder

WORKDIR /app

# Instala dependências
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copia tudo
COPY . .

# Compila
RUN cargo build --release

# Runtime stage  
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copia binário e static
COPY --from=builder /app/target/release/registros-json .
COPY --from=builder /app/static ./static

EXPOSE 3000

CMD ["./registros-json"]
