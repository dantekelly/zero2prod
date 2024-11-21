# Zero2Prod Project

This repository contains my implementation of the project from the ["Zero To Production In Rust"](https://www.zero2prod.com/) book by Luca Palmieri. While following the book's core concepts, this implementation includes several deliberate deviations to incorporate:

- Modern Rust best practices
- Updated security considerations
- Personal improvements and modifications
- Current tooling and dependencies

## Key Differences from the Book

### Current Modifications:

- Using Rust's built-in linking based on `lld` instead of the book's suggested linking configuration, as this is now the standard approach in modern Rust
- Code coverage testing has been omitted for now, as it requires additional setup that I haven't gotten around to yet.

### Planned Modifications:

- Will document additional changes here as the project progresses
- Focus on incorporating the latest security practices
- Potential updates to dependencies and their versions
- Custom improvements based on personal experience and research

## Project Structure

This is a newsletter delivery service built with Rust, following modern web development practices and cloud-native principles.

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Development

- Run `cargo watch -x check` to run the tests and linting on every change
- Keep in mind that this command can be chained with other commands, such as `-x run` to run the program and `-x test` to run the tests. Example: `cargo watch -x check -x test -x run`
- Run `cargo test` to run the tests once.
- Run `cargo fmt` to format the code.
- Run `cargo audit` to check installed crates for vulnerabilities.
