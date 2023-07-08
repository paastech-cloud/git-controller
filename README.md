# Git repo manager

## Table of Contents

  - [Table of Contents](#table-of-contents)
  - [Description](#description)
  - [Requirements](#requirements)
  - [Dependencies](#dependencies)
  - [Usage](#usage)

## Description

Git repo manager is a service to provide a simple way to manage git repositories on a machine. The communication between the client and the server is done via a GRPC Protocol.

In addition to the server implementation, a client-debug binary is also present to enhance the debugging capabilities of the system. The client-debug allows developers to perform debugging operations on the Git repo manager service.

## Requirements

Before running the server, make sure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)

## Dependencies

- [Tonic](https://github.com/hyperium/tonic) is used as the GRPC framework allowing the client to be written in any language that supports GRPC. Tonic provides the necessary tools and libraries to implement and interact with gRPC services. It allows you to define gRPC APIs, generate code from Protocol Buffers, and build both gRPC servers and clients.

- [Prost](https://github.com/tokio-rs/prost) is a Protocol Buffers implementation for Rust. It enables you to define your message and service definitions using `.proto` files, and then generate Rust code from them. This allows you to easily create a backend for your GRPC client.

- [Tokio](https://tokio.rs/) is an asynchronous runtime for the Rust programming language. It provides the building blocks needed for writing network applications. It is used to implement the server and client.

- [Dotenv]() is used to load environment variables from a `.env` file into `std::env`.

- [Log](https://docs.rs/log/0.4.14/log/) is used to provide a logging facade for the server.

- [Pretty_env_logger](https://docs.rs/pretty_env_logger/0.3.1/pretty_env_logger/) is used to provide a pretty logger for the server.


## Usage

To run the server, run the following command:

```bash
cargo run --bin repo-manager
```

To run the client-debug, run the following command:

```bash
cargo run --bin client-debug
```
