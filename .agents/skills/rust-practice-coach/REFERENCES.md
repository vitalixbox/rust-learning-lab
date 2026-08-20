# Rust Practice References

Use this catalogue to choose primary evidence, not to copy architecture. Links and repository paths were checked on 2026-08-20; inspect current docs/source again before teaching APIs or maintained-project conventions.

For every lesson, distinguish a language requirement from a broad idiom, ecosystem convention, library-specific design, or one reasonable choice. Prefer the smallest relevant example over framework internals.

## Rust standard library and official material

**Kind:** foundational library and official guidance.

- **Study:** ownership-shaped APIs, conversions, iterators, errors, threads, channels, paths, OS strings, and naming.
- **Start at:** [`std` documentation](https://doc.rust-lang.org/std/), especially `option`, `result`, `iter`, `path`, `ffi`, `sync`, `thread`, and each page's **Source** link; use the [Cargo package layout](https://doc.rust-lang.org/cargo/guide/project-layout.html) for crate conventions.
- **Also use:** [The Rust Reference](https://doc.rust-lang.org/reference/) for language rules and [The Rustonomicon](https://doc.rust-lang.org/nomicon/) only when unsafe details are relevant.
- **Do not generalize:** standard-library internals face stability, portability, and compiler constraints ordinary applications do not. Internal unsafe code is not an application template.

## Rust API Guidelines

**Kind:** rust-lang community library API guidance.

- **Study:** [naming, interoperability, documentation, predictability, flexibility, type safety, dependability, and debuggability](https://rust-lang.github.io/api-guidelines/).
- **Start at:** the checklist, then open the linked rationale for the API decision at hand.
- **Do not generalize:** the guidelines target public libraries. A private application module does not need every compatibility or extensibility measure.

## Tokio

**Kind:** async runtime library and official teaching documentation.

- **Study:** task ownership, `JoinHandle`/`JoinSet`, channels, synchronization, IO, timeouts, cancellation, and runtime behavior.
- **Start at:** the [Tokio tutorial](https://tokio.rs/tokio/tutorial), [`tokio::sync`](https://docs.rs/tokio/latest/tokio/sync/), and relevant type docs. Inspect current examples before using `select!`, shutdown, semaphore, or mutex advice.
- **Do not generalize:** Tokio-specific primitives are ecosystem choices, not Rust language requirements. Do not introduce async when blocking IO or scoped threads fit better.

## mini-redis

**Kind:** teaching application, not a production Redis implementation.

- **Study:** [server structure](https://github.com/tokio-rs/mini-redis/blob/master/src/server.rs), [shared database state](https://github.com/tokio-rs/mini-redis/blob/master/src/db.rs), [connection framing](https://github.com/tokio-rs/mini-redis/blob/master/src/connection.rs), [`cmd/`](https://github.com/tokio-rs/mini-redis/tree/master/src/cmd), and [`shutdown.rs`](https://github.com/tokio-rs/mini-redis/blob/master/src/shutdown.rs).
- **Good for:** Tokio task ownership, framing, channels, shared state, structured logging, and graceful patterns in a coherent small codebase.
- **Do not generalize:** the repository explicitly omits production Redis features. Treat its simplicity as educational and verify newer Tokio APIs in current docs.

## Axum

**Kind:** web framework library with official examples.

- **Study:** routing, typed extractors, state, `IntoResponse`, middleware, graceful shutdown, and in-process HTTP tests.
- **Start at:** [`examples/`](https://github.com/tokio-rs/axum/tree/main/examples), especially `error-handling`, `dependency-injection`, `graceful-shutdown`, `testing`, `tracing-aka-logging`, and `sqlx-postgres`; read the current [middleware guide](https://github.com/tokio-rs/axum/blob/main/axum/src/docs/middleware.md).
- **Do not generalize:** examples isolate framework features and are not prescribed application architecture. Axum extractor and router conventions are library-specific.

## Tower

**Kind:** async service abstraction and middleware library; much of the repository is framework infrastructure.

- **Study:** `Service`, `Layer`, readiness, middleware composition, timeout, buffering, and load shedding.
- **Start at:** the [Tower guides](https://github.com/tower-rs/tower/tree/master/guides), particularly `building-a-middleware-from-scratch.md`, then the docs for the exact layer being practiced.
- **Do not generalize:** hand-written `Service` futures are appropriate for reusable middleware or deeper control, not automatically for application-local Axum middleware.

## Hyper

**Kind:** low-level HTTP library and framework infrastructure.

- **Study:** connection serving, bodies, client/server boundaries, and the machinery Axum builds upon.
- **Start at:** the current [`examples/`](https://github.com/hyperium/hyper/tree/master/examples) for the precise client or server API in use.
- **Do not generalize:** Hyper's low-level types are often unnecessary in a small Axum application. Use them when the lesson is the HTTP boundary itself.

## clap

**Kind:** CLI parsing library.

- **Study:** typed command structs, subcommand enums, reusable argument groups, custom values, validation, and generated help.
- **Start at:** [`examples/tutorial_derive/`](https://github.com/clap-rs/clap/tree/master/examples/tutorial_derive), [`examples/derive_ref/`](https://github.com/clap-rs/clap/tree/master/examples/derive_ref), and the current derive docs.
- **Do not generalize:** clap types describe the command-line boundary. Do not let parser attributes become the business/domain model when conversion clarifies invariants.

## ripgrep

**Kind:** mature performance-conscious application plus reusable library crates.

- **Study:** CLI/application separation, filesystem APIs, error boundaries, parallel traversal, streaming search, and performance-aware design.
- **Start at:** [`crates/cli`](https://github.com/BurntSushi/ripgrep/tree/master/crates/cli), [`crates/core`](https://github.com/BurntSushi/ripgrep/tree/master/crates/core), and focused libraries such as [`crates/ignore`](https://github.com/BurntSushi/ripgrep/tree/master/crates/ignore) and `searcher`.
- **Do not generalize:** ripgrep's optimization, platform compatibility, and library decomposition serve an unusually performance-sensitive, mature tool. Small applications rarely need the same machinery.

## fd

**Kind:** mature CLI application.

- **Study:** a more approachable CLI layout, configuration, output, errors, exit codes, filesystem traversal, and parallel work.
- **Start at:** [`src/cli.rs`](https://github.com/sharkdp/fd/blob/master/src/cli.rs), [`config.rs`](https://github.com/sharkdp/fd/blob/master/src/config.rs), [`error.rs`](https://github.com/sharkdp/fd/blob/master/src/error.rs), [`output.rs`](https://github.com/sharkdp/fd/blob/master/src/output.rs), and [`walk.rs`](https://github.com/sharkdp/fd/blob/master/src/walk.rs).
- **Do not generalize:** mature feature flags and platform-specific behavior accumulate complexity that a learning CLI should add only when required.

## bat

**Kind:** mature CLI application with a library target and binary.

- **Study:** configuration, input/output boundaries, controller-style orchestration, library/binary separation, and terminal concerns.
- **Start at:** [`src/lib.rs`](https://github.com/sharkdp/bat/blob/master/src/lib.rs), [`src/bin/bat/`](https://github.com/sharkdp/bat/tree/master/src/bin/bat), [`config.rs`](https://github.com/sharkdp/bat/blob/master/src/config.rs), [`input.rs`](https://github.com/sharkdp/bat/blob/master/src/input.rs), and `output.rs`.
- **Do not generalize:** terminal rendering, paging, syntax assets, and cross-platform behavior dominate bat's structure; they are application requirements, not default layers.

## serde

**Kind:** foundational serialization library and advanced framework internals.

- **Study:** public trait/API stability, data model boundaries, derive ergonomics, borrowed deserialization, and feature-gated compatibility.
- **Start at:** [Serde documentation](https://serde.rs/) and examples before browsing [`serde/src`](https://github.com/serde-rs/serde/tree/master/serde/src).
- **Do not generalize:** Serde internals optimize for a vast generic ecosystem and compile-time abstraction. They are not a model for ordinary application module structure.

## tracing

**Kind:** structured diagnostics libraries and examples.

- **Study:** events versus spans, fields, instrumentation across tasks, subscriber configuration, filtering, and integration with Tower.
- **Start at:** the [tracing crate docs](https://docs.rs/tracing/latest/tracing/) and [`examples/`](https://github.com/tokio-rs/tracing/tree/master/examples), especially `attrs-basic`, `fmt`, `subscriber-filter`, and async task examples.
- **Do not generalize:** custom subscribers and collectors are library-level extension work. Most applications configure existing subscribers and instrument meaningful boundaries.

## Crossbeam

**Kind:** synchronous concurrency libraries.

- **Study:** bounded/unbounded MPMC channels, selection, scoped threads, ownership transfer, and shutdown by disconnection.
- **Start at:** [`crossbeam-channel`](https://github.com/crossbeam-rs/crossbeam/tree/main/crossbeam-channel), its `examples/`, and current crate docs.
- **Do not generalize:** richer or faster primitives do not make message passing automatically preferable to a direct loop, scoped borrow, or small mutex.

## Rayon

**Kind:** data-parallelism library with demonstrations.

- **Study:** parallel iterators, reductions, work splitting, and replacing manual worker pools for CPU-bound collection work.
- **Start at:** [Rayon crate docs](https://docs.rs/rayon/latest/rayon/) and [`rayon-demo`](https://github.com/rayon-rs/rayon/tree/main/rayon-demo).
- **Do not generalize:** parallel iterators need enough independent CPU work. Do not use Rayon for async IO or assume parallelism is faster without an appropriate baseline.

## SQLx

**Kind:** async database library and examples. The maintained repository is currently under `transact-rs/sqlx`.

- **Study:** pools, typed query boundaries, transactions, row/domain conversion, migrations, and test isolation.
- **Start at:** current [SQLx docs](https://docs.rs/sqlx/latest/sqlx/) and [`examples/sqlite`](https://github.com/transact-rs/sqlx/tree/main/examples/sqlite) for this curriculum's small persistence exercises.
- **Do not generalize:** compile-time query checking and pool setup impose environment/tooling choices. A persistence trait is not mandatory merely because SQL appears at a boundary.

## thiserror and anyhow

**Kind:** focused error-support libraries.

- **Study:** [`thiserror`](https://github.com/dtolnay/thiserror) for structured library/domain error types and [`anyhow`](https://github.com/dtolnay/anyhow) for application-level propagation and context.
- **Start at:** each crate's current README and docs, especially source preservation and context behavior.
- **Do not generalize:** the crates serve different boundaries but do not enforce a universal split. Small programs can use standard error types; libraries should not expose `anyhow::Error` merely for convenience.

## How to use a project as evidence

1. Inspect current primary docs or the default branch.
2. Name the exact file, directory, example, or API being used as evidence.
3. Explain the local problem that pattern solves.
4. State whether it is a broad idiom or project/library-specific.
5. Name at least one constraint that makes the source more complex than the exercise.
6. Borrow the idea, not a large block of code.
