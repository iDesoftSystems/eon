# Eon: A Web Development Toolkit for Rust

## What's Eon?

**Eon** is a **toolkit** designed for building **web applications in Rust**. Our primary goal is to provide useful
components that integrate natively seamlessly with other key projects in the Rust community, such as
`axum`, `sea-orm`, `serde`, and many others.

Eon simplifies common tasks, allowing you to focus on your business logic instead of reinventing the wheel. By
building on Rust's conventions and ecosystem, Eon ensures high compatibility and performance.

> [!WARNING]
>
> **Eon is in Active Development**
>
> This project is **not yet ready for production use**.
>
> For now, you can expect frequent changes, incomplete features, and the possibility of encountering bugs.

## Included Tools

Eon offers a set of essential tools for web development, designed to handle common tasks with ease:

- **HttpServer Runtime**: A simple and powerful runner for Axum services with built-in graceful shutdown and environment-based configuration.
- **Authentication**: Secure components for handling user authentication, including **JWT** support with built-in middleware and password hashing.
- **Pagination**: flexible utilities for managing and structuring paginated data, compatible with standard API response formats.
- **HTTP**: Typed builders for creating consistent and safe HTTP responses, simplifying error handling and success results.
- **ExecutableCommand**: A structured interface for designing business logic commands, promoting clean separation of concerns.
- **CommandHandler**: A robust pattern for managing command execution, ideal for building scalable and testable architectures.
- **ORM Utilities**: Seamless integration with `sea-orm`, providing common patterns and utilities for database operations.
- **Utils**: A collection of essential helper functions for everyday Rust development.

## Quick Start

### Installation

To start using Eon, add it to your `Cargo.toml`. Since we are in active development, it is recommended to use the Git repository:

```toml
[dependencies]
eon = { git = "https://github.com/iDesoftSystems/eon.git" }
```

### Usage Example: HTTP Server

Setting up a server with graceful shutdown is as simple as:

```rust
use eon::http::runtime::HttpServer;
use axum::Router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new();
    let server = HttpServer::from_env(app);
    
    server.run().await?;
    Ok(())
}
```

## Features

Eon is modular. You can enable only what you need:

- `jwt-auth`: Enables JWT authentication tools.
- `orm`: Enables base ORM utilities.
- `orm-mysql`: Enables MySQL-specific ORM support.
- `orm-sqlite`: Enables SQLite-specific ORM support.
- `chrono-ext`: Enables extended Chrono utilities.

---

Note: **Eon** is currently optimized for internal projects at **iDesoft Systems** but is built to be a general-purpose toolkit for the Rust community.
