# Projeto Georef

## Informações Principais
Este projeto tem todas as configurações e arquivos necessários para ser executado em um container **[Docker](https://www.docker.com/)**.

### Links de Instalação
**[Docker Desktop Windows](https://docs.docker.com/desktop/install/windows-install/)** <br>
**[Docker Desktop Linux](https://docs.docker.com/desktop/install/linux-install/)**

### Package Manager
Caso você esteja utilizando uma distribuição linux pode instalar o docker e docker compose via linha de comando.
Os comandos que deixei de exemplo podem gerar erros variando de máquina para máquina.

#### Ubuntu
```bash
# Add Docker's official GPG key:
sudo apt-get update
sudo apt-get install ca-certificates curl
sudo install -m 0755 -d /etc/apt/keyrings
sudo curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
sudo chmod a+r /etc/apt/keyrings/docker.asc

# Add the repository to Apt sources:
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu \
  $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
sudo apt-get update
```

#### Arch
```bash
sudo pacman -S docker docker-compose
sudo systemctl start docker.service
sudo systemctl enable docker.service
```

### Próximos passos
Agora com o docker instalado você já pode executar o container.
Abra um terminal e navegue até a pasta desse projeto, em seguida rode o comando abaixo.

```bash
docker compose up -d
```

Após isso rode ```docker ps``` e verifique se todos os containers estão rodando.
```bash
CONTAINER ID   IMAGE                 COMMAND                  CREATED      STATUS                    PORTS     NAMES
ca3db8011f55   nginx:latest          "/docker-entrypoint.…"   6 days ago   Up 38 seconds                       api-pj-georef-nginx-1
7b2f3c5dd7c7   api-pj-georef-api01   "/bin/sh -c ./target…"   6 days ago   Up 39 seconds                       api-pj-georef-api01-1
cd0988c04b9c   api-pj-georef-api02   "/bin/sh -c ./target…"   6 days ago   Up 39 seconds                       api-pj-georef-api02-1
bafac24683f2   postgres              "docker-entrypoint.s…"   6 days ago   Up 39 seconds (healthy)             api-pj-georef-db-1
```
Acima temos um exemplo de todos os containers rodando, deve haver 4 containers listados.

## Vamos para os Testes - API Methods
POST - Para criar operadora, passe o json como o exemplo.
```bash
curl https://zany-halibut-9xjwppwwxwfp7w-9999.app.github.dev/create-operadora \
    --include \
    --header "Content-Type: application/json" \
    --request "POST" \
    --data '{
      "data_operacao": "26/03/2024 13:46:159",
      "responsavel": "John Doe",
      "grupo": "Grupo A",
      "codigo_operadora": 30,
      "operadora": "ABC Telecom",
      "razao_social": "ABC Company",
      "cnpj": "1234567890",
      "email": "teste@teste.com",
      "telefone": "123-456-7890"
    }'
```

POST - Para criar praça, passe o json como o exemplo.
```bash
# Create variables with random values
longitude=$(shuf -i 180-180 -n 1)
latitude=$(shuf -i 90-90 -n 1)
id_operadora=$(openssl rand -hex 4)
km=$(shuf -i 0-100 -n 1)
codigo_praca=$(shuf -i 0-180 -n 1)

# Send POST request
curl http://localhost:9999/create-praca \
    --include \
    --header "Content-Type: application/json" \
    --request "POST" \
    --data '{
    "longitude": '$longitude',
    "latitude": '$latitude',
    "id_operadora": "'"$id_operadora"'",
    "nome": "random_nome",
    "situacao": "random_situacao",
    "rodovia": "random_rodovia",
    "km": '$km',
    "sentido": "random_sentido",
    "cidade": "random_cidade",
    "estado": "random_estado",
    "codigo_praca": '$codigo_praca',
    "orientacao": "random_orientacao",
    "tipo": "random_tipo",
    "jurisdicao": "random_jurisdicao",
    "cobranca_especial": false,
    "categoria": "random_categoria",
    "data_de_alteracao": "random_data_de_alteracao",
    "razao_social": "ABC Company",
    "cnpj": "1234567890",
    "email": "teste@teste.com",
    "telefone": "123-456-7890"
}'
```

```bash
curl http://localhost:9999/upload_stats -X POST -F 'file=@Cargo.toml'
```

```bash
curl http://localhost:9999/upload_stats -X POST -F 'file=@Cargo.toml' -F 'layout=advanced'
```

## Executar comandos direto no banco de dados(Terminal)

Abri o terminal do container do banco de dados
```bash
docker exec -it api-pj-georef-db-1 sh
```

```bash
psql -U root -h localhost -p 5432 -d pj_georef
```