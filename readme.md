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

Eon offers a set of essential tools for web development:

- **Authentication**: Components for handling user authentication.
- **Pagination**: Utilities for managing and structuring paginated data.
- **HTTP**: Types and builders for creating simple and safe HTTP responses.
- **ExecutableCommand**: An interface for designing and executing commands in a structured way.
- **CommandHandler**: A pattern for managing the execution of your commands, ideal for event-driven architectures.
- **ORM**: A collection of utilities for working with sea-orm.
- **Utils**: A collection of useful utilities.

## Getting Started

To start using Eon, add the following packages to your `Cargo.toml` file:

```toml
[dependencies]
eon = { git = "https://github.com/iDesoftSystems/eon.git", version = "x.y.z" }
```

Note: It's important to use the development version from the Git repository, as the project has not yet been published
on `crates.io`.
