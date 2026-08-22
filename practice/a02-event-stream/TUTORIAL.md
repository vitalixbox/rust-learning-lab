# Tutorial: Reduce a Mixed Event Stream

Work through one checkpoint at a time. Write the code yourself, run the named
check, and stop to answer the checkpoint question before moving on. The
tutorial intentionally gives shapes and invariants rather than a completed
reducer.

## 0. Make the design explicit

Before creating `src/lib.rs`, sketch two types on paper or in comments.

The event enum needs these cases:

- deposit: account ID and amount;
- withdrawal: account ID, amount, and whether it was approved;
- freeze: account ID and an owned reason;
- closure: account ID;
- audit note: an owned message but no account ID.

The summary needs deposited and withdrawn totals, a declined-withdrawal count,
a large-withdrawal flag, an optional latest freeze reason, and a closed flag.

Decide whether the reducer should borrow or consume its events. The exercise
requires consumption because the latest freeze reason can then move from an
event into the summary without cloning.

**Checkpoint:** Which fields should be `String`, `Option<String>`, integers, or
booleans? Why does only one summary field need `Option`?

## 1. Create the library target

Create `src/lib.rs` inside this package. Add only the event and summary types
from your sketch. Derive traits that make test failures readable and equality
assertions possible.

Give the summary a zero-value constructor. `Default` is appropriate if every
field has an unsurprising empty value; an explicit constructor is also valid.

Run:

```console
cargo check --manifest-path practice/a02-event-stream/Cargo.toml
```

**Checkpoint:** Can callers accidentally construct a contradictory event, such
as an audit note with an account ID? If so, improve the enum shape.

## 2. Establish the empty-stream behavior

Add the reducer boundary and implement only enough to return an empty summary.
A concrete function accepting an owned `Vec<AccountEvent>` is sufficient; this
exercise does not need an `IntoIterator` generic.

Write the empty-stream test first. Assert every field, either separately or by
comparing with the summary's empty value.

Run just that test:

```console
cargo test --manifest-path practice/a02-event-stream/Cargo.toml empty
```

**Checkpoint:** At the function boundary, what proves that events can be moved
out of the input rather than borrowed?

## 3. Separate irrelevant events from relevant events

Iterate by value. For each event, determine whether it has an account ID.
Audit notes do not. This is the natural place to practice `let else`: extract
the optional ID, or continue when none exists.

Then ignore IDs other than the requested target. Keep this filtering near the
top of the loop so the update logic can assume it is handling the right
account.

You will need an exhaustive `match` somewhere to map event variants to an
optional account ID. It can live in a small method on the event enum or locally
inside the reducer. Choose based on whether that operation reads like reusable
behavior or a reducer detail.

Add a test containing only another account's events and audit notes. Its result
must equal the empty summary.

**Checkpoint:** Does extracting the ID borrow the event briefly, leaving the
whole event available to move afterward? Avoid cloning as a workaround.

## 4. Reduce deposits and withdrawals

Extend the event-processing match:

- add every target deposit to the deposited total;
- add an approved withdrawal to the withdrawn total;
- count a declined withdrawal without changing either money total.

Use destructuring to bind only the payload needed by each arm. Use a match
guard for the approved/declined distinction rather than nesting another large
conditional inside one withdrawal arm.

Add one mixed-stream test with at least one deposit, one approved withdrawal,
and one declined withdrawal.

**Checkpoint:** Are approved and declined withdrawals visibly separate cases?
Would a reader understand which one changes the total without tracing nested
branches?

## 5. Detect a large approved withdrawal

The flag becomes true when an approved withdrawal is at least 1,000 units and
must never return to false. Practice `matches!` for this boolean classification.
You can classify the event before consuming it in the update match, or classify
the relevant values inside the withdrawal branch.

Be deliberate about `&event` versus `event`: matching by value too early may
move owned fields and prevent later use. The compiler error is useful evidence
about where ownership changed.

Test the boundary values 999 and 1,000, and include a declined withdrawal over
1,000 to prove approval is part of the rule.

**Checkpoint:** Does the predicate encode both conditions in one readable
place? Is the flag monotonic?

## 6. Retain the latest freeze reason

A freeze is a sparse special case with an owned payload. Practice `if let` to
recognize it, move its reason into the summary, and then continue or otherwise
avoid processing the same event twice.

Process two freezes for the target account and verify that the second reason
replaces the first. Do not clone either reason.

**Checkpoint:** Why does assignment to `Option<String>` safely drop the older
reason? At which exact pattern does the new `String` move?

## 7. Record closure

Set the closed flag when a target closure appears. Keep it true if later events
are present; this reducer summarizes observations and does not validate a
lifecycle.

This branch should stay small. If matching an event as a boolean reads more
clearly than destructuring an unused payload, `matches!` is a reasonable
choice. Do not force it if you already used `matches!` more naturally for the
large-withdrawal rule.

Add closure to the mixed-stream test.

**Checkpoint:** Are you accidentally imposing event-order rules that the brief
did not request?

## 8. Review pattern choice

Before polishing, locate each construct and state its job:

- exhaustive `match`: choose behavior by event variant;
- match guard: refine one variant using a payload condition;
- `let else`: reject an irrelevant shape early;
- `if let`: handle one payload-carrying special case;
- `matches!`: answer a boolean classification question.

If two constructs do the same job awkwardly, simplify first and ask for a
review. The purpose is judgment, not maximizing syntax variety.

## 9. Run the completion checks

Run the package checks from the repository root:

```console
cargo fmt --manifest-path practice/a02-event-stream/Cargo.toml --check
cargo clippy --manifest-path practice/a02-event-stream/Cargo.toml -- -D warnings
cargo test --manifest-path practice/a02-event-stream/Cargo.toml
```

Finally, inspect the reducer for unnecessary `clone`, wildcard match arms, and
nested conditionals that hide the event cases.

When these checks pass, request a review. The exercise is complete only after
the behavior, Clippy findings, design explanation, and Rustacean review have
all been considered.

## If you get stuck

Ask for the smallest useful hint and name the checkpoint. Useful requests are:

- “Direction hint for checkpoint 3.”
- “Which Rust mechanism helps with the partial-move error in checkpoint 5?”
- “Show me a non-solution syntax example of `matches!` with a guard.”

This keeps the next decision yours without requiring you to debug in isolation.
