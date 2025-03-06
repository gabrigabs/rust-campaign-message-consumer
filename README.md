## 🚀 Visão Geral

Esta aplicação é um consumidor de mensagens, é usado para processar as mensagens que a [API de Gerenciamento de Campanhas](https://github.com/gabrigabs/api-campaign-management) cria, processando as mensagens enviadas ao RabbitMQ e garantir que sejam armazenadas e rastreadas corretamente.

## 🔧 Configuração

### Pré-requisitos

- Rust (versão estável mais recente)
- Cargo (gerenciador de pacotes do Rust)
- RabbitMQ em execução
- MongoDB
- PostgreSQL
- As mesmas configurações de ambiente usadas na API de Gerenciamento de Campanhas

### Instalação de Dependências

```bash
cargo build
```

### Variáveis de Ambiente

Você deve configurar as variáveis de ambiente com os mesmos valores usados na [API de Gerenciamento de Campanhas](https://github.com/gabrigabs/api-campaign-management) Copie o arquivo `.env.example` para `.env` e ajuste conforme necessário:

> ⚠️ **AVISO:** Sempre insira no final da sua url mongodb a query ?authSource=admin sem ela a aplicação não irá conseguir interagir com o mongodb

```bash
# MongoDB
MONGODB_URI=mongodb://root:root@localhost:27017/messages?authSource=admin
MONGODB_DB_NAME=messages

# RabbitMQ
RABBITMQ_URL=amqp://guest:guest@localhost:5672
RABBITMQ_QUEUE=campaigns

# PostgreSQL
POSTGRES_HOST=localhost
POSTGRES_PORT=5432
POSTGRES_DB=campaigns
POSTGRES_USER=postgres
POSTGRES_PASSWORD=postgres
POSTGRES_SSL=false

# Application
LOG_LEVEL=info
```

## 🚀 Executando o Aplicativo

### Build

```bash
cargo build
```
### Execução

```bash
cargo run
```