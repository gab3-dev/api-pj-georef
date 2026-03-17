FROM rust:1.90

RUN apt-get update -yqq && \
    apt-get install -yqq cmake g++ && \
    cargo install cargo-watch

WORKDIR /actix

# cache de dependências
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main(){}" > src/main.rs
RUN cargo build

# remove dummy
RUN rm -rf src

EXPOSE 8080

CMD ["cargo", "watch", "-x", "run"]
