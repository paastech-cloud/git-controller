## BUILDER
##
FROM rust:slim AS builder

RUN apt-get update && apt-get install -y libfuse-dev pkg-config fuse3
COPY . .
RUN cargo build --release


## RUNNNER
##
FROM debian:bullseye-slim AS runner

RUN apt-get update && apt-get install -y libfuse-dev pkg-config fuse3

RUN useradd -ms /bin/bash newuser
USER newuser
WORKDIR /home/newuser

COPY --from=builder ./target/release/ssh-keys-fuse /usr/local/bin
COPY fuse.conf /etc/fuse.conf

RUN mkdir /home/newuser/.ssh

CMD ["ssh-keys-fuse"]
