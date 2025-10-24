# Correções para Health Check Failando no Railway

## Problemas Identificados e Soluções

### ❌ Problema 1: Health Check muito curto
**Antes:** 100 segundos
**Depois:** 300 segundos (5 minutos)
**Motivo:** Railway precisa de mais tempo para compilar Rust em release mode

### ❌ Problema 2: Health check dependia do MongoDB
**Antes:** Health check retornava informações do MongoDB
**Depois:** Health check simples que sempre retorna 200 OK
**Motivo:** Se MongoDB falhar na conexão, health check não deveria falhar

### ❌ Problema 3: Comando de start incorreto
**Antes:** `cargo run --release` (recompila toda vez)
**Depois:** `./target/release/registros-json` (usa binário já compilado)
**Motivo:** Mais rápido e eficiente

## Arquivos Modificados

### 1. `railway.toml`
```toml
[build]
builder = "NIXPACKS"

[deploy]
healthcheckPath = "/api/health"
healthcheckTimeout = 300
```

### 2. `nixpacks.toml` (NOVO)
```toml
[phases.setup]
nixPkgs = ["openssl", "pkg-config"]

[phases.build]
cmds = ["cargo build --release"]

[start]
cmd = "./target/release/registros-json"
```

### 3. `src/main.rs` - Health Check Simplificado
```rust
async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({
        "status": "ok",
        "service": "registros-json",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}
```

## Commit e Deploy

```bash
git add .
git commit -m "Fix: Health check timeout e configuração Railway"
git push origin main
```

## Verificação

Após o deploy, verifique:
1. Logs no Railway mostram: "👂 Escutando em 0.0.0.0:XXXX"
2. Health check retorna 200 OK
3. Frontend acessível
4. API funcionando

## Troubleshooting

Se ainda falhar:

1. **Verifique MONGODB_URI**
   - URI deve estar correta e acessível
   - Whitelist do IP do Railway no MongoDB Atlas

2. **Verifique Logs**
   - Procure por erros de conexão
   - Verifique se a porta está sendo lida corretamente

3. **Teste Local**
   ```bash
   PORT=8080 cargo run --release
   curl http://localhost:8080/api/health
   ```
