# Rust Learning Lab

A collection of small, independent Rust projects for practicing the language.

## Projects

- `hello` — a minimal “Hello, world!” program.
- `projects/gcd` — finds the greatest common divisor of positive integers.
- `projects/actix-gcd` — serves the GCD calculator as a web application.
- `projects/mandelbrot` — renders the Mandelbrot set to a PNG image.
- `projects/quickreplace` — performs regular-expression search and replace between files.

Each project is its own Cargo package. Run commands from the project directory, or use `--manifest-path` from the repository root.

## Getting started

Install a stable Rust toolchain, then clone the repository and try a project:

```sh
cargo run --manifest-path hello/Cargo.toml
cargo run --manifest-path projects/gcd/Cargo.toml -- 42 56
```

Run tests and checks from the repository root:

```sh
cargo test --manifest-path hello/Cargo.toml
cargo test --manifest-path projects/gcd/Cargo.toml
cargo clippy --manifest-path hello/Cargo.toml
cargo clippy --manifest-path projects/gcd/Cargo.toml
```
