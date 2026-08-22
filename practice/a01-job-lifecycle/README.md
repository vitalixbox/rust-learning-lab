# A01 — Model a job lifecycle

## Task

Build a small job-lifecycle model for background work. A job has an identity and
is in exactly one of four states: queued, running, succeeded, or failed. Apply
explicit transitions and report attempts that are not valid from the current
state.

## Why this exercise exists

This exercise trains data-carrying enums, exhaustive matching, and representing
state-specific data so invalid combinations cannot be constructed accidentally.

## Real-world connection

Job runners and schedulers commonly model work as a closed lifecycle. Using a
data-carrying enum for that lifecycle follows the same broad Rust idiom seen in
standard types such as `Result`: each case carries only the data meaningful to
that case, and callers handle the cases explicitly. This is a widely accepted
idiom, not a language requirement.

## Constraints

- Use only the standard library.
- Store state-specific data in the relevant enum variant. Do not model the
  lifecycle as boolean flags or as a struct full of optional fields.
- Support only these transitions: queued to running, and running to either
  succeeded or failed.
- Reject every other transition with a typed error; do not panic.
- Keep the design concrete: no traits, generics, async work, persistence, or
  wall-clock handling.

## Acceptance criteria

- A job always has exactly one lifecycle state.
- Starting a queued job records a worker name and supplied start sequence.
- Completing a running job records a supplied finish sequence and artifact
  name.
- Failing a running job records a supplied finish sequence and reason.
- An invalid transition reports both the attempted action and current state.
- Rejection does not make the original job unavailable to the caller.
- Tests cover the success path, failure path, and at least two rejected
  transitions.
- `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo test` pass.

## Suggested crates

None. The standard library is sufficient, and supplied integer sequences keep
time handling outside this exercise.

## Timebox

45–60 minutes.
