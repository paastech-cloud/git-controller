FROM rust:slim-bookworm AS build

WORKDIR /app

# install protobuf
RUN apt update && apt install -y -qq protobuf-compiler libprotobuf-dev

COPY Cargo.* .

RUN cargo fetch

COPY src src

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

# add proto user
RUN useradd proto

COPY --from=build --chown=proto:proto /app/target/release/server /app

USER proto

CMD [ "./repo-manager" ]
