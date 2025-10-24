# Configuração para Deploy no Railway

## ✅ Checklist de Deploy

### 1. Arquivos de Configuração Criados
- ✅ `railway.toml` - Configuração do Railway
- ✅ `Procfile` - Comando de inicialização
- ✅ `.dockerignore` - Otimização do build
- ✅ `README.md` - Documentação

### 2. Configuração da Porta
- ✅ A aplicação usa a variável `PORT` do ambiente
- ✅ Porta padrão local: 3000
- ✅ Railway define automaticamente a `PORT` em produção
- ✅ Bind em `0.0.0.0` (aceita conexões externas)

### 3. Variáveis de Ambiente Necessárias no Railway

Configure no painel do Railway:

```
MONGODB_URI=mongodb+srv://usuario:senha@cluster.mongodb.net/
DATABASE_NAME=registros_db
COLLECTION_NAME=registros
```

**Importante:** Não precisa definir `PORT` - o Railway define automaticamente!

### 4. Health Check
- Endpoint: `/api/health`
- Timeout: 100 segundos
- Retorna status JSON

### 5. Build
- Builder: NIXPACKS (auto-detecta Rust)
- Comando: `cargo run --release`
- Restart: ON_FAILURE (até 10 tentativas)

## 🚀 Passos para Deploy

1. **Faça commit das mudanças:**
   ```bash
   git add .
   git commit -m "Configuração para deploy no Railway"
   git push origin main
   ```

2. **No Railway:**
   - Crie um novo projeto
   - Conecte ao repositório GitHub
   - Configure as variáveis de ambiente
   - Deploy automático será iniciado

3. **Verifique o Deploy:**
   - Acesse o health check: `https://seu-app.railway.app/api/health`
   - Teste o frontend: `https://seu-app.railway.app/`
   - Logs disponíveis no painel do Railway

## 📊 Monitoramento

- Health check automático a cada requisição
- Restart automático em caso de falha
- Logs em tempo real no painel Railway

## 🔒 Segurança

- ✅ Variáveis de ambiente protegidas
- ✅ HTTPS automático pelo Railway
- ✅ `.env` no `.gitignore` (não versionado)
