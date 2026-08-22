# A02 — Reduce a mixed event stream

## Task

Build a reducer that consumes account events and produces a summary for one
target account. The stream can contain deposits, approved or declined
withdrawals, account freezes, closures, and audit notes that belong to no
account.

The summary should report deposited and withdrawn totals, the number of
declined withdrawals, whether a large approved withdrawal occurred, the latest
freeze reason, and whether the account was closed.

## Why this exercise exists

This exercise trains choosing among destructuring, match guards, `if let`,
`let else`, and `matches!`. The goal is not to use the most compact syntax, but
to make each branch's filtering and data extraction obvious.

## Real-world connection

Reducers over typed events are common in audit processing, telemetry, and
state reconstruction. Rust's data-carrying enums and exhaustive `match` make
heterogeneous input explicit. This is a widely accepted idiom; the exact
summary design is one reasonable application choice.

## Constraints

- Use only the standard library.
- Represent event kinds with one data-carrying enum; do not use strings to
  identify event types.
- Consume the event collection rather than cloning owned event data.
- Ignore account-scoped events for accounts other than the target.
- Audit notes have no account ID and do not affect the summary.
- A large withdrawal means an approved withdrawal of at least 1,000 units.
- Exercise all of `match`, a match guard, `if let`, `let else`, and `matches!`
  where each remains readable. Do not contort every branch into one construct.
- Use integer amounts; currency precision is outside this exercise.
- Keep the design concrete: no traits, generics, async work, or external crates.

## Acceptance criteria

- Deposits for the target account are added to the deposited total.
- Only approved target-account withdrawals are added to the withdrawn total.
- Declined target-account withdrawals are counted but do not change totals.
- The summary records whether at least one large approved withdrawal occurred.
- If the target account is frozen more than once, the latest reason is retained.
- A target-account closure is recorded.
- Events for other accounts and audit notes do not affect the summary.
- An empty stream produces a summary containing zero totals, zero declined
  withdrawals, no freeze reason, and false flags.
- Tests cover a mixed stream, irrelevant events, repeated freezes, and an empty
  stream.
- `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo test` pass.

## Suggested crates

None. The standard library has everything needed for this reducer.

## Timebox

30–45 minutes.
