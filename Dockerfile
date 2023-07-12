FROM rust:slim-bookworm AS build

WORKDIR /app

# install protobuf
RUN apt update && apt install -y -qq protobuf-compiler libprotobuf-dev

COPY Cargo.* .

RUN cargo fetch

COPY src src

RUN cargo build --release --bin server

FROM debian:bookworm-slim

WORKDIR /app

RUN apt update && apt install -y -qq git

RUN apt autoremove -y -qq && \
    apt clean -y -qq && \
    rm -rf /var/lib/apt/lists/*

COPY --from=build /app/target/release/server /app

CMD [ "./server" ]
