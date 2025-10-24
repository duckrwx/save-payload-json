# Etapa 1: Build (compilação)
FROM rust:latest as builder

WORKDIR /app

# Copia arquivos de dependências
COPY Cargo.toml Cargo.lock ./

# Cria projeto dummy para cachear dependências
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copia código real
COPY src ./src
COPY static ./static

# Compila a aplicação
RUN touch src/main.rs && cargo build --release

# Etapa 2: Runtime (execução)
FROM debian:sid-slim

# Instala dependências necessárias
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copia executável da etapa anterior
COPY --from=builder /app/target/release/registros-json /app/registros-json
COPY --from=builder /app/static /app/static

# Porta
EXPOSE 8080

# Variável de ambiente para porta
ENV PORT=8080

# Executa
CMD ["/app/registros-json"]