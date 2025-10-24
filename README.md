# Registros JSON - Sistema de Gerenciamento de Registros

Sistema de registro de dados com MongoDB e API REST desenvolvido em Rust.

## 🚀 Deploy no Railway

### Variáveis de Ambiente Necessárias

Configure as seguintes variáveis no Railway:

```
MONGODB_URI=mongodb+srv://usuario:senha@cluster.mongodb.net/
DATABASE_NAME=registros_db
COLLECTION_NAME=registros
```

### Configuração Automática

O Railway detecta automaticamente:
- ✅ Linguagem: Rust
- ✅ Build: Cargo
- ✅ Porta: Variável `PORT` (definida automaticamente pelo Railway)

### Health Check

O endpoint de health check está disponível em:
```
GET /api/health
```

Retorna:
```json
{
  "status": "ok",
  "mode": "mongodb",
  "timestamp": "2025-10-24T..."
}
```

## 📦 Endpoints da API

### Adicionar Registro
```
POST /api/adicionar
Content-Type: application/json

{
  "estado": "SP",
  "responsavel": "Nome do Responsável",
  "payload": "{\"dados\": \"json aqui\"}"
}
```

### Resposta de Sucesso
```json
{
  "status": "sucesso",
  "mensagem": "Registro salvo com ID: 507f1f77bcf86cd799439011"
}
```

## 🛠️ Desenvolvimento Local

### Pré-requisitos
- Rust 1.70+
- MongoDB (local ou Atlas)

### Instalação

1. Clone o repositório
```bash
git clone https://github.com/duckrwx/save-payload-json.git
cd save-payload-json
```

2. Crie o arquivo `.env`
```env
MONGODB_URI=mongodb://localhost:27017
DATABASE_NAME=registros_db
COLLECTION_NAME=registros
PORT=3000
```

3. Execute o projeto
```bash
cargo run
```

O servidor estará disponível em `http://localhost:3000`

## 🏗️ Estrutura do Projeto

```
src/
├── main.rs           # Servidor HTTP e rotas
├── models.rs         # Modelos de dados (Estado, Regional, Registro)
├── task_manager.rs   # Lógica de processamento
├── storage.rs        # Trait e implementações de storage
└── error.rs          # Gerenciamento de erros

static/
├── index.html        # Interface web
├── script.js         # Lógica do frontend
└── style.css         # Estilos
```

## 📝 Licença

MIT License
