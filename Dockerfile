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

COPY --from=build /app/target/release/server /app

CMD [ "./server" ]
