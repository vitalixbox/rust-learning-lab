# Rust Learning Lab

A collection of small, independent Rust projects for practicing the language.

## Projects

- `hello` — a minimal “Hello, world!” program.
- `gcd` — a command-line program that finds the greatest common divisor of a list of positive integers.

Each project is its own Cargo package. Run commands from the project directory, or use `--manifest-path` from the repository root.

## Getting started

Install a stable Rust toolchain, then clone the repository and try a project:

```sh
cargo run --manifest-path hello/Cargo.toml
cargo run --manifest-path gcd/Cargo.toml -- 42 56
```

Run all current tests and checks from the repository root:

```sh
cargo test --manifest-path hello/Cargo.toml
cargo test --manifest-path gcd/Cargo.toml
cargo clippy --manifest-path hello/Cargo.toml
cargo clippy --manifest-path gcd/Cargo.toml
```
