# Projeto Georef / BGM

API e aplicação web para gestão de dados georreferenciados de pedágios no Brasil. O projeto mantém cadastro de operadoras, praças de pedágio, tipos de tarifa e tarifas cobradas por praça.

A regra principal de tarifas é preservar histórico: quando uma tarifa é atualizada, o registro anterior é marcado como `Inativo` e uma nova tarifa `Ativo` é criada.

## Stack

- Backend: Rust, Actix Web, SQLx e PostgreSQL
- Frontend: Angular 19, Angular Material, PrimeNG e AG Grid
- Banco: PostgreSQL
- Proxy: Nginx
- Testes E2E: Playwright
- Orquestração: Docker Compose

## Domínio

- `operadora`: concessionárias/operadoras de pedágio.
- `pedagio`: praças de pedágio, vinculadas a uma operadora.
- `tipo_tarifa`: categorias de tarifa por tipo de veículo/rodagem/eixos.
- `tarifas`: valores cobrados por praça e tipo de tarifa.

## Estrutura

```text
src/
  auth/        autenticação JWT, usuários e seed de administrador
  operadora/   handlers, modelos e queries de operadoras
  pedagio/     handlers, modelos e queries de pedágios
  tarifa/      handlers, modelos e queries de tipos de tarifa e tarifas
  utils/       importação CSV e upload de arquivos
web/           aplicação Angular
tests/         testes de integração da API
init.sql       schema inicial e dados de exemplo
nginx*.conf    configuração do proxy
```

## Execução Local Com Docker

Pré-requisitos:

- Docker
- Docker Compose

Suba o ambiente de desenvolvimento:

```bash
docker compose -f docker-compose.yaml up --build
```

Serviços principais:

- API: `http://localhost:8081`
- Nginx: `http://localhost:9999`
- Frontend: `http://localhost:4200`, quando servido pelo Angular
- PostgreSQL: `localhost:5432`

O compose local usa `network_mode: host`. Em Linux isso permite que API, frontend, Nginx e PostgreSQL conversem via `localhost`.

Administrador inicial em desenvolvimento:

- E-mail: `admin@bgm.com`
- Senha padrão: valor de `ADMIN_PASSWORD` ou `admin123`

## Execução Local Sem Docker

Pré-requisitos:

- Rust estável
- PostgreSQL disponível
- Node.js para o frontend

Compile a API:

```bash
cargo build
```

Execute a API:

```bash
cargo run
```

Execute o frontend:

```bash
cd web
npm install
npm start
```

## Variáveis De Ambiente

A API aceita `DATABASE_URL` ou as variáveis separadas de conexão.

| Variável               | Padrão em desenvolvimento       | Descrição                                                             |
| ---------------------- | ------------------------------- | --------------------------------------------------------------------- |
| `DATABASE_URL`         | vazio                           | URL completa do PostgreSQL. Se definida, tem prioridade.              |
| `DB_HOST`              | `localhost`                     | Host do PostgreSQL.                                                   |
| `DB_PORT`              | `5432`                          | Porta do PostgreSQL.                                                  |
| `DB_NAME`              | `pj_georef`                     | Nome do banco.                                                        |
| `DB_USER`              | `root`                          | Usuário do banco.                                                     |
| `DB_PASSWORD`          | `1234`                          | Senha do banco.                                                       |
| `HTTP_PORT`            | `9999`                          | Porta HTTP da API quando executada diretamente.                       |
| `POOL_SIZE`            | `125`                           | Tamanho máximo do pool SQLx.                                          |
| `JWT_SECRET`           | `chave_secreta_desenvolvimento` | Segredo usado para assinar tokens JWT.                                |
| `JWT_EXPIRATION_HOURS` | `8`                             | Validade dos tokens em horas.                                         |
| `CORS_ALLOWED_ORIGINS` | `*`                             | Origens liberadas para CORS. Em produção é obrigatório.               |
| `UPLOAD_DIR`           | `./tmp`                         | Diretório onde a API salva uploads CSV.                               |
| `DB_UPLOAD_DIR`        | `/uploaded`                     | Caminho do mesmo arquivo visto pelo container do banco.               |
| `SEED_ADMIN`           | `false`                         | Cria administrador inicial quando `true`.                             |
| `ADMIN_EMAIL`          | `admin@bgm.com`                 | E-mail do administrador inicial.                                      |
| `ADMIN_NAME`           | `Administrador`                 | Nome do administrador inicial.                                        |
| `ADMIN_PASSWORD`       | vazio                           | Senha do administrador inicial. Obrigatória quando `SEED_ADMIN=true`. |

Em produção, variáveis sensíveis não têm fallback e devem ser definidas explicitamente.

## Autenticação

A API usa JWT Bearer token.

Login:

```http
POST /api/auth/login
```

Payload:

```json
{
  "email": "admin@bgm.com",
  "senha": "admin123"
}
```

Use o token retornado nos endpoints protegidos:

```http
Authorization: Bearer <token>
```

Perfis:

- `admin`: pode criar, atualizar e importar dados.
- `user`: pode consultar dados.

## Endpoints Principais

Todos os endpoints ficam sob `/api`.

### Autenticação E Usuários

| Método | Rota              | Descrição                     |
| ------ | ----------------- | ----------------------------- |
| `POST` | `/api/auth/login` | Login e emissão de JWT.       |
| `POST` | `/api/usuarios`   | Cria usuário. Requer admin.   |
| `GET`  | `/api/usuarios`   | Lista usuários. Requer admin. |

### Operadoras

| Método | Rota                                 | Descrição                         |
| ------ | ------------------------------------ | --------------------------------- |
| `GET`  | `/api/operadoras`                    | Lista operadoras.                 |
| `GET`  | `/api/operadoras/{codigo_operadora}` | Busca operadora por código.       |
| `POST` | `/api/operadoras`                    | Cria operadora. Requer admin.     |
| `PUT`  | `/api/operadoras/{codigo_operadora}` | Atualiza operadora. Requer admin. |

Filtros em `/api/operadoras`: `q`, `codigo_operadora`, `operadora`, `grupo`, `cnpj`, `email`.

### Pedágios

| Método | Rota                         | Descrição                     |
| ------ | ---------------------------- | ----------------------------- |
| `GET`  | `/api/pedagios`              | Lista praças de pedágio.      |
| `GET`  | `/api/pedagios/{id_pedagio}` | Busca praça por ID.           |
| `POST` | `/api/pedagios`              | Cria praça. Requer admin.     |
| `PUT`  | `/api/pedagios/{id_pedagio}` | Atualiza praça. Requer admin. |

Filtros em `/api/pedagios`: `q`, `id_pedagio`, `codigo_operadora`, `nome`, `situacao`, `rodovia`, `cidade`, `estado`, `codigo`.

### Tarifas

| Método | Rota                       | Descrição                                                         |
| ------ | -------------------------- | ----------------------------------------------------------------- |
| `GET`  | `/api/tipos-tarifa`        | Lista tipos de tarifa.                                            |
| `POST` | `/api/tipos-tarifa`        | Cria tipo de tarifa. Requer admin.                                |
| `GET`  | `/api/tarifas`             | Lista apenas tarifas ativas.                                      |
| `GET`  | `/api/tarifas/{id_tarifa}` | Busca tarifa por ID.                                              |
| `POST` | `/api/tarifas`             | Cria tarifa. Requer admin.                                        |
| `PUT`  | `/api/tarifas/{id_tarifa}` | Inativa a tarifa anterior e cria nova versão ativa. Requer admin. |

Filtros em `/api/tipos-tarifa`: `q`, `id_tipo_tarifa`, `id_padrao_tarifa`, `descricao`, `rodagem`, `eixos`.

Filtros em `/api/tarifas`: `q`, `id_tarifa`, `id_tipo_tarifa`, `id_pedagio`, `codigo_operadora`, `situacao`, `tipo`, `descricao`, `nome`, `operadora`, `incluir_inativas`.

Por padrão, `/api/tarifas` retorna apenas `situacao=Ativo`. Para consultar histórico, use `situacao=Inativo` ou `incluir_inativas=true`.

### Importação CSV

| Método | Rota                      | Descrição                                 |
| ------ | ------------------------- | ----------------------------------------- |
| `POST` | `/api/imports/operadoras` | Importa operadoras via CSV. Requer admin. |
| `POST` | `/api/imports/pedagios`   | Importa pedágios via CSV. Requer admin.   |
| `POST` | `/api/imports/tarifas`    | Importa tarifas via CSV. Requer admin.    |

As rotas antigas (`/api/create-*`, `/api/get-*`, `/api/update-*`, `/api/importar-*`) ainda existem para compatibilidade, mas as rotas REST acima são as preferenciais.

## Regra De Histórico De Tarifas

Ao atualizar uma tarifa existente por `PUT /api/tarifas/{id_tarifa}`:

1. A tarifa encontrada por `id_tarifa` é marcada como `Inativo`.
2. Os dados históricos do registro antigo são preservados.
3. Uma nova linha é inserida em `tarifas` com novo `id_tarifa` e `situacao = 'Ativo'`.
4. `id_tipo_tarifa` e `id_pedagio` permanecem os mesmos da tarifa anterior.

Na importação CSV de tarifas, quando o arquivo contém um `id_tarifa` já existente, o comportamento é o mesmo: a tarifa anterior é inativada e uma nova versão ativa é inserida.

## Importação CSV

Os endpoints de importação recebem `multipart/form-data` com campo `file`.

Exemplo:

```bash
curl -X POST http://localhost:9999/api/imports/tarifas \
  -H "Authorization: Bearer <token>" \
  -F "file=@tarifas.csv"
```

Os arquivos usam `;` como delimitador e devem conter cabeçalho. Para tarifas, a coluna `id_tarifa` é obrigatória.

Durante a importação, a API salva o arquivo em `UPLOAD_DIR` e o PostgreSQL lê o arquivo pelo caminho equivalente em `DB_UPLOAD_DIR`, usando `COPY`.

## Testes

Testes de integração da API:

```bash
cargo test
```

Os testes usam PostgreSQL. Por padrão tentam conectar em:

```text
postgres://root:1234@localhost:5432/pj_georef
```

É possível sobrescrever com:

- `DB_HOST`
- `DB_PORT`
- `DB_NAME`
- `DB_USER`
- `DB_PASSWORD`

Testes E2E web com ambiente Docker isolado:

```bash
cd web
npm run e2e:docker
```

Limpar ambiente E2E:

```bash
cd web
npm run e2e:docker:down
```

O ambiente E2E usa `.env.test`, banco separado, volumes próprios e seed automático de administrador.

## Produção

Use o compose de produção para API, PostgreSQL e Nginx:

```bash
cp .env.production.example .env.production
docker compose --env-file .env.production -f docker-compose-production.yaml up -d --build
```

Variáveis mínimas em `.env.production`:

- `POSTGRES_USER`
- `POSTGRES_PASSWORD`
- `POSTGRES_DB`
- `JWT_SECRET`
- `CORS_ALLOWED_ORIGINS`

O Nginx publica a API em:

```text
http://<host>:9999/api/*
```

Para criar um administrador inicial no primeiro deploy:

```env
SEED_ADMIN=true
ADMIN_EMAIL=admin@bgm.com
ADMIN_NAME=Administrador
ADMIN_PASSWORD=<senha-forte>
```

Depois do primeiro deploy, volte `SEED_ADMIN=false`.

## Comandos Úteis

Build release da API:

```bash
make target
```

Acessar o banco local via Docker:

```bash
docker exec -it api-pj-georef-db-1 psql -U root -d pj_georef
```

Ver containers:

```bash
docker ps
```
