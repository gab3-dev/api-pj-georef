# Rust API-RESTful with Actix
## Rust Lang
Rust is a multi-paradigm, general-purpose programming language that emphasizes performance, type safety, and concurrency. It enforces memory safety, meaning that all references point to valid memory, without requiring the use of automated memory management techniques such as garbage collection. To simultaneously enforce memory safety and prevent data races, its "borrow checker" tracks the object lifetime of all references in a program during compilation.
**[Learn more about Rust](https://www.rust-lang.org/)**.
## Actix Framework
Actix Web is a powerful, pragmatic, and extremely fast web framework for Rust
**[Learn more about Actix](https://actix.rs/)**.
## Creating my Rust RESTful API
To contribute to my Back-End studies, I decided to develop two APIs in different technologies. In the future I intend to use both APIs developed in a personal project related to games. 
For now both are being done in the REST architecture, because it is the most common and easier to work with, later one of the architectures will change to RPC.
## Run
***For this tutorial, I conclude that you already has installed the Rust Lang. If it isn't your case, you can download and install it at this link:*** **[Rust Lang Download](https://www.rust-lang.org/tools/install)**
<br>If you are using a linux distribution and have the curl installed, you can install Rust just running this command:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
<br>First you must build the project with this command:
```
cargo build
```
Now just run with:
```
cargo run
```
## API Methods
GET
```bash
curl http://localhost:9999/ \
    --header "Content-Type: application/json" \
    --request "GET"
```

POST
```bash
curl http://localhost:9999/create-operadora \
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

```bash
curl http://localhost:9999/upload_stats -X POST -F 'file=@Cargo.toml'
```

```bash
curl http://localhost:9999/upload_stats -X POST -F 'file=@Cargo.toml' -F 'layout=advanced'
```