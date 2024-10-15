# Road to Mastering Rust

This guide provides resources and examples to help you learn Rust, covering basic syntax, asynchronous programming, REST API development, database interaction, and gRPC.

## Basic Syntax

- **Rust by Example**: Learn the fundamentals of Rust up to Section 9, which covers functions.
  - [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

## Asynchronous Programming

- **Learning Resources**:
  - [Asynchronous Programming in Rust - YouTube Video](https://www.youtube.com/watch?v=K8LNPYNvT-U)
  - [Channels in Rust (Part 1) - Medium Article](https://medium.com/@disserman/channels-in-rust-part-1-d28a07bf782c)

- **Tokio Framework**: Explore asynchronous programming using the Tokio framework, including a project to build a mini Redis clone.
  - [Tokio](https://tokio.rs/)
  - **Example**: [lib.rs](https://github.com/Armingodiz/get-rusty/blob/master/src/lib.rs)
  - You will probably need to read [shared-mutable-state](https://draft.ryhl.io/blog/shared-mutable-state/), because the shared-state section in the tokio mini redis is hard to follow

## REST API Development

- **Actix Web Framework**: Utilize Actix Web for building RESTful APIs in Rust.
  - [Actix Web](https://actix.rs/)
  - **Example**: [main.rs](https://github.com/Armingodiz/get-rusty/blob/master/src/main.rs)

## Database Interaction

- **SQLx Library**: Perform asynchronous database operations using SQLx.
  - [SQLx Documentation](https://docs.rs/sqlx/latest/sqlx/)
  - **Example**: [handlers.rs](https://github.com/Armingodiz/get-rusty/blob/master/src/handlers.rs)

## gRPC

- **Tonic gRPC**: Implement gRPC services in Rust with the Tonic library.
  - [Tonic gRPC Tutorial](https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md)
  - **Example**: *TODO*
