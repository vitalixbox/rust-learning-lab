# Rust Practice Roadmap

This is a menu, not a syllabus. Reorder, adapt, repeat, or skip exercises using the evidence in `PROGRESS.md`. Keep every exercise to 1–3 central ideas and split it if the timebox grows beyond two hours.

Each implementation belongs in an independent Cargo package at `practice/<id>-<slug>/`. The learner writes implementation source.

## Track A — Everyday Rust idioms

### A01 — Model a job lifecycle

- **Target idioms:** data-carrying enums, exhaustive `match`, illegal states made unrepresentable
- **Time:** 45–60 minutes
- **Prerequisites:** basic structs, enums, and `Result`
- **Description:** Model a small background job that can be queued, running, succeeded, or failed, then apply validated state transitions and report rejected transitions.

### A02 — Reduce a mixed event stream

- **Target idioms:** destructuring, match guards, `if let`, `let else`, `matches!`
- **Time:** 30–45 minutes
- **Prerequisites:** A01 or equivalent enum experience
- **Description:** Consume heterogeneous account events and update a compact summary while choosing the clearest pattern form for each branch.

### A03 — Overlay optional profile settings

- **Target idioms:** `Option::map`, `and_then`, `filter`, `or_else`, `transpose`
- **Time:** 45–60 minutes
- **Prerequisites:** `Option` and closures
- **Description:** Merge partial user settings without placeholder values or deeply nested conditionals, preserving the distinction between absent and invalid input.

### A04 — Parse a batch with useful failures

- **Target idioms:** `Result` transformations, `?`, `map_err`, error enums, collecting results
- **Time:** 60–75 minutes
- **Prerequisites:** A03 helpful
- **Description:** Parse line-oriented records, attach line context at the application boundary, and choose whether to stop at the first failure or report several.

### A05 — Build a log-summary pipeline

- **Target idioms:** iterators, adapters, `fold`, `collect`, explicit-loop tradeoffs
- **Time:** 45–60 minutes
- **Prerequisites:** collections and closures
- **Description:** Filter, transform, group, and summarize records; keep a loop where stateful control flow is clearer and explain the choice.

### A06 — Design text and filesystem boundaries

- **Target idioms:** `String`/`&str`, `PathBuf`/`&Path`, `OsString`/`&OsStr`, borrowed input and owned storage
- **Time:** 60–75 minutes
- **Prerequisites:** borrowing fundamentals
- **Description:** Design a file-selection API that accepts platform paths without assuming UTF-8 and allocates only when values must be retained.

### A07 — Validate values with newtypes

- **Target idioms:** invariants, `FromStr`, `TryFrom`, `From`, conversion errors
- **Time:** 60–75 minutes
- **Prerequisites:** A04
- **Description:** Introduce validated identifiers and port/range values so downstream code operates on trusted types rather than repeatedly checking primitives.

### A08 — Choose closed or open behavior

- **Target idioms:** enum versus trait, concrete types, closure parameters, extension traits only when justified
- **Time:** 60–90 minutes
- **Prerequisites:** A01, closures, basic traits
- **Description:** Implement a small selection policy twice: a closed enum of known policies and caller-supplied behavior, then compare exhaustiveness, extensibility, and complexity.

## Track B — CLI applications

### B01 — Typed commands and subcommands

- **Target idioms:** `clap` derive, command structs, subcommand enums, boundary-to-domain conversion
- **Time:** 60–75 minutes
- **Prerequisites:** A01, A07 helpful
- **Description:** Define a small task CLI with two subcommands and keep argument parsing separate from the operation it selects.

### B02 — Parse validated CLI values

- **Target idioms:** custom value types, `FromStr`, validation timing, actionable diagnostics
- **Time:** 45–60 minutes
- **Prerequisites:** A07, B01
- **Description:** Replace primitive CLI fields with validated domain values and decide which checks belong in parsing versus business logic.

### B03 — Read stdin or a file

- **Target idioms:** `PathBuf` ownership, `&Path` APIs, `Read`/`BufRead`, boundary enums
- **Time:** 60–75 minutes
- **Prerequisites:** A06, B01
- **Description:** Model input as stdin or a path, process it through one core operation, and avoid conflating OS paths with UTF-8 text.

### B04 — Separate human and machine output

- **Target idioms:** output model, serde serialization, stdout/stderr, exit status, application errors
- **Time:** 60–90 minutes
- **Prerequisites:** B01, A04
- **Description:** Add human-readable and JSON output modes with stable behavior, clean error reporting, and meaningful process exit codes.

### B05 — Apply configuration precedence and tracing

- **Target idioms:** partial configuration, CLI/env/file/default precedence, validation after merge, `tracing` fields
- **Time:** 75–90 minutes
- **Prerequisites:** A03, B01
- **Description:** Merge four configuration sources into one validated runtime config and emit structured diagnostics without mixing logs into machine output.

### B06 — Interrupt and test a CLI

- **Target idioms:** Ctrl-C ownership, cooperative stop flag/channel, process-level integration tests, observable contracts
- **Time:** 75–105 minutes
- **Prerequisites:** B04; C03 or D06 helpful depending on implementation
- **Description:** Make a long-running command stop predictably and test arguments, output streams, and exit status as a real process.

## Track C — Synchronous concurrency

### C01 — Borrow into scoped threads

- **Target idioms:** `std::thread::scope`, disjoint borrowing, joining, panic propagation
- **Time:** 45–60 minutes
- **Prerequisites:** ownership and slices
- **Description:** Process borrowed chunks in parallel without `Arc` or `'static`, then explain why scoped lifetime guarantees make it safe.

### C02 — Share read-mostly state

- **Target idioms:** `Arc<T>`, `Mutex<T>`, lock scope, immutable versus mutable sharing
- **Time:** 60–75 minutes
- **Prerequisites:** C01
- **Description:** Serve several worker threads from shared immutable configuration and protect only a small mutable statistics structure.

### C03 — Give a worker a bounded channel

- **Target idioms:** ownership transfer, bounded Crossbeam channels, sender/receiver closure, backpressure
- **Time:** 60–75 minutes
- **Prerequisites:** A01; C01 helpful
- **Description:** Run one dedicated worker that owns a resource and accepts commands until all senders are dropped.

### C04 — Fan out and fan in

- **Target idioms:** cloned receivers or work distribution, result channel, worker lifecycle, simple pool design
- **Time:** 75–90 minutes
- **Prerequisites:** C03
- **Description:** Distribute independent jobs across a fixed set of workers and gather tagged results without losing ownership or hanging shutdown.

### C05 — Replace manual workers with Rayon

- **Target idioms:** parallel iterators, associative reductions, data parallelism, granularity
- **Time:** 45–60 minutes
- **Prerequisites:** A05, C04 helpful
- **Description:** Parallelize a CPU-bound transformation with Rayon, compare it with sequential iterators, and identify why a hand-built pool is unnecessary here.

### C06 — Count cheaply and justify concurrency

- **Target idioms:** atomics and ordering at an introductory level, contention, sequential baseline, when not to parallelize
- **Time:** 60–75 minutes
- **Prerequisites:** C02, C05
- **Description:** Add low-contention counters to concurrent work, then measure or reason about whether concurrency improves the workload at all.

## Track D — Async Rust and Tokio

### D01 — Spawn owned work

- **Target idioms:** `tokio::spawn`, `'static` task ownership, `move`, `JoinHandle`
- **Time:** 45–60 minutes
- **Prerequisites:** ownership and basic `async`/`await`
- **Description:** Spawn a small set of independent tasks, decide what each task owns, await results, and surface join failures deliberately.

### D02 — Coordinate a dynamic task set

- **Target idioms:** `JoinSet`, completion order, task errors, abort-on-drop considerations
- **Time:** 60–75 minutes
- **Prerequisites:** D01
- **Description:** Launch a runtime-sized set of operations, consume completions as they arrive, and define behavior when one task fails.

### D03 — Put state behind an owner task

- **Target idioms:** bounded Tokio `mpsc`, command enum, receiver ownership, state-owner task
- **Time:** 60–75 minutes
- **Prerequisites:** A01, D01
- **Description:** Move mutable state into one task and expose operations by sending typed commands instead of sharing an async mutex.

### D04 — Request and respond with `oneshot`

- **Target idioms:** `oneshot`, reply handles inside commands, cancellation through dropped endpoints
- **Time:** 60–75 minutes
- **Prerequisites:** D03
- **Description:** Extend the state owner so callers receive per-request results and must handle an owner task that has stopped.

### D05 — Race work, timeout, and cancellation

- **Target idioms:** `tokio::select!`, `timeout`, losing branches, cancellation safety
- **Time:** 75–90 minutes
- **Prerequisites:** D01, D04
- **Description:** Race an operation against a deadline and shutdown signal, then reason explicitly about what happens to partially completed work.

### D06 — Shut down a task tree

- **Target idioms:** cooperative cancellation, closing channels, draining tasks, async resource cleanup
- **Time:** 75–105 minutes
- **Prerequisites:** D02, D05
- **Description:** Give a small parent/worker task tree a graceful shutdown path that stops intake, lets owned resources close, and joins children.

### D07 — Bound concurrent async work

- **Target idioms:** `Semaphore`, permit ownership, `JoinSet`, failure and permit release
- **Time:** 60–75 minutes
- **Prerequisites:** D02
- **Description:** Process many simulated remote calls while enforcing a strict concurrency ceiling and retaining task outcomes.

### D08 — Build a backpressured stream pipeline

- **Target idioms:** streams, bounded buffers, producer/consumer rates, backpressure
- **Time:** 75–90 minutes
- **Prerequisites:** D03, D07
- **Description:** Connect an async producer, transformer, and sink with bounded capacity and observe how slow consumption affects upstream work.

### D09 — Choose the right mutex in async code

- **Target idioms:** `std::sync::Mutex` versus `tokio::sync::Mutex`, no `.await` under a sync guard, shared state versus owner task
- **Time:** 60–75 minutes
- **Prerequisites:** C02, D03
- **Description:** Implement two tiny shared-state cases and justify a standard mutex, async mutex, or dedicated owner for each.

## Track E — Networking and web services

### E01 — Handle TCP connections per task

- **Target idioms:** `TcpListener`, connection ownership, accept loop, task lifecycle
- **Time:** 60–75 minutes
- **Prerequisites:** D01
- **Description:** Build a small line-echo server where each spawned task owns its socket and the accept loop handles connection errors deliberately.

### E02 — Add a framed protocol

- **Target idioms:** buffering, frame boundaries, parsing errors versus IO errors, owned messages
- **Time:** 75–90 minutes
- **Prerequisites:** E01, A04
- **Description:** Replace raw line echo with a tiny length- or delimiter-framed request/response protocol and explicit protocol errors.

### E03 — Route typed HTTP input

- **Target idioms:** Axum router, path/query extractors, typed JSON request and response
- **Time:** 60–75 minutes
- **Prerequisites:** D01, serde basics
- **Description:** Create a small router with one read and one write route, validating the transport shape without yet introducing shared state.

### E04 — Pass state and separate the domain operation

- **Target idioms:** Axum `State`, cloneable application handle, thin handlers, concrete dependency passing
- **Time:** 60–75 minutes
- **Prerequisites:** E03, F01 helpful
- **Description:** Give the router application state and move a meaningful transformation out of the handler without creating a framework of service traits.

### E05 — Map domain failures to HTTP

- **Target idioms:** domain error enum, transport error type, `IntoResponse`, boundary conversion
- **Time:** 60–75 minutes
- **Prerequisites:** A04, E04
- **Description:** Keep domain failures independent of HTTP and convert them into stable status codes and response bodies at the handler boundary.

### E06 — Compose middleware and tracing

- **Target idioms:** Axum middleware versus Tower layer, request spans, timeout placement, middleware ordering
- **Time:** 75–90 minutes
- **Prerequisites:** E03, tracing basics
- **Description:** Add request IDs, structured tracing, and a timeout using the simplest middleware abstraction that fits each concern.

### E07 — Shut down and test in process

- **Target idioms:** graceful HTTP shutdown, router-as-service tests, `ServiceExt::oneshot`, no external port
- **Time:** 75–90 minutes
- **Prerequisites:** D06, E05
- **Description:** Add a shutdown signal and test routes, state, and error mapping directly through the service without starting an external server.

### E08 — Add a small SQLite boundary

- **Target idioms:** SQLx pool in application state, query boundary, database errors, test isolation
- **Time:** 90–120 minutes
- **Prerequisites:** E04, E05
- **Description:** Persist one domain record in SQLite while keeping SQL and database-specific failures at a narrow application boundary.

## Track F — Application and domain structure

### F01 — Package validated domain types

- **Target idioms:** domain structs, newtypes, constructors, invariants, private fields
- **Time:** 60–75 minutes
- **Prerequisites:** A07
- **Description:** Model a small reservation or inventory concept so invalid values cannot enter normal business operations.

### F02 — Express a state machine with an enum

- **Target idioms:** state enums, transition methods, exhaustive behavior, state-dependent data
- **Time:** 60–75 minutes
- **Prerequisites:** A01, F01
- **Description:** Represent an order-like lifecycle, including data that exists only in particular states, without boolean flags or nullable fields.

### F03 — Separate IO from a pure command reducer

- **Target idioms:** commands and events, pure transformation, effect boundary, deterministic tests
- **Time:** 75–90 minutes
- **Prerequisites:** F02
- **Description:** Turn validated commands into new state and emitted events in a pure core, leaving loading, saving, and output outside.

### F04 — Pass a dependency without architecture theater

- **Target idioms:** concrete parameter, generic bound, small boundary trait, test seam
- **Time:** 60–90 minutes
- **Prerequisites:** F01, basic traits
- **Description:** Add one external lookup to a domain operation and compare a concrete type, a generic parameter, and a narrow trait at the call boundary.

### F05 — Remove unnecessary dynamic sharing

- **Target idioms:** composition, owned dependencies, `Arc<dyn Trait>` tradeoffs, synchronous versus async boundaries
- **Time:** 60–75 minutes
- **Prerequisites:** F04, C02 or D09 helpful
- **Description:** Refactor a deliberately over-abstracted design sketch into the smallest set of concrete values and shared owners that the requirements need.

### F06 — Build a library crate with a thin binary

- **Target idioms:** `lib.rs`/`main.rs` boundary, module organization, visibility, application error boundary
- **Time:** 75–90 minutes
- **Prerequisites:** F03
- **Description:** Package reusable business behavior in a library target while the binary performs configuration, IO wiring, reporting, and exit handling.

## Track G — Library and API design

### G01 — Accept borrowed input, return owned output

- **Target idioms:** borrowing-friendly signatures, lifetime elision, owned results, avoiding needless clones
- **Time:** 45–60 minutes
- **Prerequisites:** A06
- **Description:** Design a small normalization/search library whose callers can pass existing data without surrendering ownership.

### G02 — Choose concrete paths or `AsRef<Path>`

- **Target idioms:** `&Path`, `PathBuf`, `impl AsRef<Path>`, API flexibility versus noise
- **Time:** 45–60 minutes
- **Prerequisites:** A06, G01
- **Description:** Expose a few filesystem operations and decide independently where a concrete borrowed path or generic convenience is appropriate.

### G03 — Expose iteration without exposing storage

- **Target idioms:** `impl Iterator`, custom iterator, extension trait when justified, collection encapsulation
- **Time:** 60–90 minutes
- **Prerequisites:** A05, G01
- **Description:** Let callers traverse filtered domain values lazily, then implement one small iterator type or narrowly useful extension method.

### G04 — Design constructors and error surfaces

- **Target idioms:** invariants, constructor naming, library error type, `source`, non-exhaustive evolution considerations
- **Time:** 60–75 minutes
- **Prerequisites:** A04, A07
- **Description:** Design a public type that validates construction and reports predictable errors without exposing implementation details.

### G05 — Compare static and dynamic dispatch

- **Target idioms:** generic bounds, trait objects, object safety, zero-cost abstraction, API complexity
- **Time:** 75–90 minutes
- **Prerequisites:** A08, G01
- **Description:** Offer one operation through a generic and a trait-object interface, then compare binary shape, heterogeneous storage, ergonomics, and extensibility.

### G06 — Finish a small public API

- **Target idioms:** builder/configuration API, visibility, sealed details, documentation examples, public API tests
- **Time:** 90–120 minutes
- **Prerequisites:** G01, G04; G03 helpful
- **Description:** Polish a small library surface with a justified builder, focused docs, compileable examples, and tests written as an external caller.
