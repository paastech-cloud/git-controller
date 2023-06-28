# GIT REPO MANAGER (Gitsprout)

## Description

Git repo manager is a service to provide a simple way to manage git repositories on a machine.
The communication between the client and the server is done via a GRPC Protocol. Here is the server implementation.

[Tonic](https://github.com/hyperium/tonic) is used as the GRPC framework allowing the client to be written in any language that supports GRPC. Tonic provides the necessary tools and libraries to implement and interact with gRPC services. It allows you to define gRPC APIs, generate code from Protocol Buffers, and build both gRPC servers and clients.

[Prost](https://github.com/tokio-rs/prost) is a Protocol Buffers implementation for Rust. It enables you to define your message and service definitions using `.proto` files, and then generate Rust code from them. This allows you to easily create a backend for your GRPC client.

[Tokio](https://tokio.rs/) is an asynchronous runtime for the Rust programming language. It provides the building blocks needed for writing network applications. It is used to implement the server and client.


## Requirements

Before running the server, you need to install the following dependencies:

- [Rust](https://www.rust-lang.org/tools/install)
- [Tonic Dependencies](https://github.com/hyperium/tonic)


