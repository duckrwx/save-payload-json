# Railway Deploy - Configuração

## Opção 1: Build Automático (Nixpacks)
Railway detecta automaticamente Rust e faz o build.

**Arquivos necessários:**
- ✅ `Cargo.toml`
- ✅ `rust-toolchain.toml`
- ✅ `Procfile`

**Variáveis de ambiente:**
```
MONGODB_URI=mongodb+srv://...
DATABASE_NAME=registros_db
COLLECTION_NAME=registros
```

## Opção 2: Dockerfile (Recomendado)
Build mais rápido e previsível.

**Para usar Dockerfile no Railway:**
1. Railway detecta `Dockerfile` automaticamente
2. Build multi-stage reduz tamanho da imagem
3. Health check integrado

**Comando:**
Railway usa automaticamente: `docker build -t app .`

## Health Check
Endpoint: `/api/health`

Resposta:
```json
{
  "status": "ok",
  "service": "registros-json",
  "timestamp": "2025-10-24T..."
}
```

## Deploy
```bash
git add .
git commit -m "Add Dockerfile e health check"
git push origin main
```

Railway fará deploy automaticamente.
