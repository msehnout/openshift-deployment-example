FROM rust:latest

RUN groupadd -r user && useradd --no-log-init -r -g user user
RUN cargo new /central-node
WORKDIR /central-node

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN cargo build --release

USER user
EXPOSE 8080
CMD ["/central-node/target/release/central-node"]
