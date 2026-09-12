# Senior Engineer Interview Preparation

A mastery-based course for senior software engineering interviews, using Rust and building on a Java background. The default specialization is backend/distributed systems; adjust it when your target role is clearer. There are no deadlines or daily streaks.

## Start here

Open **[CURRENT_GOAL.md](CURRENT_GOAL.md)**. It contains the only active assignment, an exact next action, and a place to record where you stopped.

```powershell
cargo test --lib exercises::binary_search::
```

G018 (binary search) is intentionally unfinished, so its tests initially fail with a `todo!` panic. Your task is to implement it. Say **review** for correctness and Rust feedback; say **next** to set up the next exercise immediately. Pending explanations are saved for later and do not block an explicit request to advance. You do not need to read the whole course first.

## Course materials

- [Course](course/COURSE.md): progression, practice sequence, and advancement criteria.
- [Interview playbook](course/PLAYBOOK.md): coding, design, leadership, mock interviews, and readiness rubric.
- [Rust bridge](course/RUST_BRIDGE.md): essential Rust concepts for a Java developer.
- [Progress](course/PROGRESS.md): completed goals, review queue, and evidence of readiness.
- [Sources](course/SOURCES.md): official references and company-specific calibration.

## Repository map and commands

`src/exercises/` holds exercises and unit tests. `src/lib.rs` registers the exercise modules. `src/main.rs` displays the starting instructions. `course/` holds learning materials. `AGENTS.md` and `note_for_codex` describe contributor and coaching preferences.

```powershell
cargo run                              # Show where to start
cargo test                             # Run all activated exercise tests
cargo fmt --check                      # Check formatting
cargo clippy --all-targets -- -D warnings # Check lint warnings
```

Use `cargo fmt` to fix formatting. The current package has no external dependencies. Future exercises are scaffolded only when activated, so an unfinished backlog will not bury your current test results.

Ask for “a syntax hint” or “an algorithm hint.” Coaching should expose the smallest useful hint first.

| Command | Action |
| --- | --- |
| `review` | Critique only the current exercise and run its checks. |
| `next` | Set up one next exercise and its tests. |
| `stop` | Save your place and one tiny next action, then pause. |
| `resume` | Return to that action without a quiz or catch-up work. |

Commands are case-insensitive. You can stop again after the small resumption action; finishing an exercise is not required.
