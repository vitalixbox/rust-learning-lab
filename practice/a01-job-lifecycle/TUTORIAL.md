# Tutorial: Implement a Job Lifecycle

This walkthrough implements A01 with consuming transitions. Each transition
takes ownership of a `Job` and returns either its new state or a rejection that
contains the original job.

That design makes state changes explicit and lets enum payloads move into the
next state without cloning.

## 1. Remove Cargo's placeholder

Open `src/lib.rs` and remove the generated `add` function and its test. The
following steps all go into that file.

## 2. Model the lifecycle

Start with one enum whose variants contain only valid data for that state:

```rust
#[derive(Debug, PartialEq, Eq)]
pub enum JobState {
    Queued {
        queued_at: u64,
    },
    Running {
        queued_at: u64,
        started_at: u64,
        worker: String,
    },
    Succeeded {
        queued_at: u64,
        started_at: u64,
        finished_at: u64,
        worker: String,
        artifact: String,
    },
    Failed {
        queued_at: u64,
        started_at: u64,
        finished_at: u64,
        worker: String,
        reason: String,
    },
}
```

There are no independent `running` and `failed` flags, so contradictory states
cannot be constructed. Data is carried forward when it remains useful; for
example, a succeeded job still records which worker ran it.

The integer sequences are supplied by the caller. This keeps clocks and date
libraries outside the exercise.

## 3. Name states and attempted actions

Errors need to report the current state without copying the state's potentially
large payload. Add two small, data-free enums:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobStateKind {
    Queued,
    Running,
    Succeeded,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionAction {
    Start,
    Succeed,
    Fail,
}
```

`JobStateKind` is a compact description used for reporting. `JobState` remains
the authoritative state and owns the real data.

Add an exhaustive conversion from a state to its kind:

```rust
impl JobState {
    fn kind(&self) -> JobStateKind {
        match self {
            JobState::Queued { .. } => JobStateKind::Queued,
            JobState::Running { .. } => JobStateKind::Running,
            JobState::Succeeded { .. } => JobStateKind::Succeeded,
            JobState::Failed { .. } => JobStateKind::Failed,
        }
    }
}
```

Do not use a wildcard arm here. If another lifecycle variant is added later,
the compiler should identify this match as something that needs review.

## 4. Define the job and rejection types

The job identity exists in every state, so it belongs outside `JobState`:

```rust
#[derive(Debug, PartialEq, Eq)]
pub struct Job {
    id: u64,
    state: JobState,
}
```

Use one type for the error information and another for ownership recovery:

```rust
#[derive(Debug, PartialEq, Eq)]
pub struct TransitionError {
    pub attempted: TransitionAction,
    pub current: JobStateKind,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RejectedTransition {
    pub job: Job,
    pub error: TransitionError,
}
```

A transition consumes `Job`. Returning only `TransitionError` would lose the
job on failure. `RejectedTransition` returns both the error and ownership of the
unchanged job.

## 5. Centralize rejection construction

Add this private helper:

```rust
fn reject(
    id: u64,
    state: JobState,
    attempted: TransitionAction,
) -> RejectedTransition {
    let current = state.kind();

    RejectedTransition {
        job: Job { id, state },
        error: TransitionError { attempted, current },
    }
}
```

`state.kind()` borrows the state briefly. After that borrow ends, the complete
state moves back into the rejected job. No clone is needed.

## 6. Construct and inspect jobs

Begin the `Job` implementation with a constructor and borrowed accessors:

```rust
impl Job {
    pub fn new(id: u64, queued_at: u64) -> Self {
        Self {
            id,
            state: JobState::Queued { queued_at },
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn state(&self) -> &JobState {
        &self.state
    }
}
```

The fields stay private so callers cannot replace the state without using a
validated transition. The state accessor borrows rather than clones.

## 7. Implement queued to running

Add `start` inside the existing `impl Job` block:

```rust
pub fn start(
    self,
    worker: String,
    started_at: u64,
) -> Result<Self, RejectedTransition> {
    let Job { id, state } = self;

    match state {
        JobState::Queued { queued_at } => Ok(Job {
            id,
            state: JobState::Running {
                queued_at,
                started_at,
                worker,
            },
        }),
        state => Err(reject(id, state, TransitionAction::Start)),
    }
}
```

`let Job { id, state } = self` destructures the owned job. The match then owns
`state`, so it can move `queued_at` into the running variant. The rejection arm
binds the unmatched state and moves it back into a `Job`.

## 8. Implement the terminal transitions

Add `succeed` to the same `impl Job` block:

```rust
pub fn succeed(
    self,
    artifact: String,
    finished_at: u64,
) -> Result<Self, RejectedTransition> {
    let Job { id, state } = self;

    match state {
        JobState::Running {
            queued_at,
            started_at,
            worker,
        } => Ok(Job {
            id,
            state: JobState::Succeeded {
                queued_at,
                started_at,
                finished_at,
                worker,
                artifact,
            },
        }),
        state => Err(reject(id, state, TransitionAction::Succeed)),
    }
}
```

Then add `fail`:

```rust
pub fn fail(
    self,
    reason: String,
    finished_at: u64,
) -> Result<Self, RejectedTransition> {
    let Job { id, state } = self;

    match state {
        JobState::Running {
            queued_at,
            started_at,
            worker,
        } => Ok(Job {
            id,
            state: JobState::Failed {
                queued_at,
                started_at,
                finished_at,
                worker,
                reason,
            },
        }),
        state => Err(reject(id, state, TransitionAction::Fail)),
    }
}
```

Both methods move the owned `worker` string from `Running` into the terminal
state. Calling `clone()` would compile, but it would allocate unnecessarily.

The only permitted matches are now:

```text
Queued  --start-->   Running
Running --succeed--> Succeeded
Running --fail-->    Failed
```

Every other state/action pair reaches a typed rejection.

## 9. Test successful transitions

Add a test module below the implementation:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queued_job_can_succeed() {
        let job = Job::new(7, 10)
            .start("worker-a".to_owned(), 20)
            .unwrap()
            .succeed("build.tar".to_owned(), 30)
            .unwrap();

        assert_eq!(job.id(), 7);

        match job.state() {
            JobState::Succeeded {
                queued_at,
                started_at,
                finished_at,
                worker,
                artifact,
            } => {
                assert_eq!(*queued_at, 10);
                assert_eq!(*started_at, 20);
                assert_eq!(*finished_at, 30);
                assert_eq!(worker, "worker-a");
                assert_eq!(artifact, "build.tar");
            }
            state => panic!("expected succeeded, got {state:?}"),
        }
    }

    #[test]
    fn running_job_can_fail() {
        let job = Job::new(8, 100)
            .start("worker-b".to_owned(), 110)
            .unwrap()
            .fail("compiler crashed".to_owned(), 120)
            .unwrap();

        match job.state() {
            JobState::Failed { reason, .. } => {
                assert_eq!(reason, "compiler crashed");
            }
            state => panic!("expected failed, got {state:?}"),
        }
    }
}
```

These tests destructure the terminal variants and verify their payloads. The
fallback arms make a wrong state produce a useful diagnostic.

## 10. Test rejected transitions

Add two more tests inside the same test module:

```rust
#[test]
fn queued_job_cannot_succeed() {
    let rejection = Job::new(9, 200)
        .succeed("impossible.tar".to_owned(), 210)
        .unwrap_err();

    assert_eq!(
        rejection.error,
        TransitionError {
            attempted: TransitionAction::Succeed,
            current: JobStateKind::Queued,
        }
    );
    assert_eq!(rejection.job.id(), 9);
    assert!(matches!(
        rejection.job.state(),
        JobState::Queued { queued_at: 200 }
    ));
}

#[test]
fn succeeded_job_cannot_start_again() {
    let job = Job::new(10, 300)
        .start("worker-c".to_owned(), 310)
        .unwrap()
        .succeed("release.tar".to_owned(), 320)
        .unwrap();

    let rejection = job.start("worker-d".to_owned(), 330).unwrap_err();

    assert_eq!(rejection.error.attempted, TransitionAction::Start);
    assert_eq!(rejection.error.current, JobStateKind::Succeeded);
    assert_eq!(rejection.job.id(), 10);
    assert!(matches!(
        rejection.job.state(),
        JobState::Succeeded { .. }
    ));
}
```

The important assertion is not only that an error occurred. Each test also
uses `rejection.job`, proving that rejection preserved ownership and state.

## 11. Run the checks

From the repository root:

```bash
cargo fmt --manifest-path practice/a01-job-lifecycle/Cargo.toml --check
cargo clippy --manifest-path practice/a01-job-lifecycle/Cargo.toml -- -D warnings
cargo test --manifest-path practice/a01-job-lifecycle/Cargo.toml
```

If formatting fails, run the same `cargo fmt` command without `--check`, then
check again.

## What this implementation teaches

- A data-carrying enum expresses mutually exclusive states and their data.
- Exhaustive `match` makes lifecycle changes visible to the compiler.
- Consuming `self` turns a transition into an owned value transformation.
- Owned payloads can move between variants without allocation or cloning.
- A rejection wrapper can return both structured error information and the
  original value.

In a larger system, job history might be stored as separate events rather than
copied into every later state. That is a production modeling choice, not needed
for this ownership exercise.
