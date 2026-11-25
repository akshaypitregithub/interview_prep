# Repository Guidelines

## Project Structure & Module Organization

This repository supports incremental data structures and algorithms (DSA) interview preparation in Rust.

- `CURRENT_GOAL.md`: Authoritative active exercise, contract, scoped check commands, and resumption bookmark.
- `note_for_codex`: Learner background, coaching rules, and command definitions.
- `course/`: Curriculum (`COURSE.md`), workflows (`PLAYBOOK.md`), progress log (`PROGRESS.md`), Java-to-Rust guide (`RUST_BRIDGE.md`), references (`SOURCES.md`), and per-goal reviews (`course/reviews/G###.md`).
- `src/lib.rs` and `src/exercises/`: Independently testable exercise modules (one focused file per exercise, registered in `src/exercises/mod.rs`).
- `src/main.rs`: Entry point printing quick-start instructions.
- `Cargo.toml` / `Cargo.lock`: Rust 2024 package with no external dependencies. Generated build output belongs in `target/`, which Git ignores.

## Build, Test, and Development Commands

Run these commands from the repository root with a Rust toolchain supporting edition 2024:

- `cargo test --lib exercises::<module_name>::`: Run only the active exercise's tests (required scope for `review`).
- `rustfmt --edition 2024 --check src/exercises/<module_name>.rs`: Check formatting for the active exercise file only.
- `clippy-driver --edition=2024 --test src/exercises/<module_name>.rs --crate-name <goal>_review --emit=metadata --out-dir target -D warnings`: Run standalone Clippy on the active exercise file only.
- `cargo build` / `cargo run` / `cargo test` / `cargo fmt`: Full-package commands when explicitly needed.

## Coding Style & Naming Conventions

Use standard rustfmt formatting and four-space indentation. Use `snake_case` for functions, modules, and variables; `PascalCase` for types and traits; and `SCREAMING_SNAKE_CASE` for constants. Prefer small functions with explicit inputs and outputs. Explain algorithm invariants and time/space complexity where useful.

## Testing Guidelines

Use Rust's built-in `#[test]` framework, placing unit tests in a `#[cfg(test)] mod tests` block alongside each exercise. Name tests by behavior, such as `returns_none_when_target_is_absent`. Cover normal cases and applicable boundaries: empty inputs, single elements, duplicates, missing values, and numeric limits.

## Commit & Pull Request Guidelines

Use concise, imperative messages such as `Add binary search exercise`. Keep changes focused.

## Learning & Agent Instructions

Follow `note_for_codex` and `course/PLAYBOOK.md` at all times:

- **Session startup:** Whenever a new session begins or the user sends a course command, read `CURRENT_GOAL.md`, `note_for_codex`, and `course/PLAYBOOK.md` (and the active exercise file in `src/exercises/`) before responding.
- **Course commands (case-insensitive):**
  - `start` / `resume`: Follow the **Resume** workflow in `course/PLAYBOOK.md` — read `CURRENT_GOAL.md` and the active exercise file, restore the same goal, mark the session active in `CURRENT_GOAL.md` and `course/PROGRESS.md`, and give at most one sentence of context plus one tiny starting action. Do not run a quiz, automatic checks, catch-up work, or supply a solution.
  - `stop`: Follow the **Stop** workflow in `course/PLAYBOOK.md` — save an honest bookmark and one tiny next action in `CURRENT_GOAL.md`, record the pause in `course/PROGRESS.md`, preserve learner code/tests, and stop briefly without running checks or asking questions.
  - `review`: Follow the **Repeatable implementation review** and **Patchwork assessment** workflows in `course/PLAYBOOK.md` — scope tests, formatting, and Clippy to the active exercise only, preserve learner code/tests, flag patchwork as invalid for acceptance, append to `course/reviews/<goal-id>.md`, update `CURRENT_GOAL.md` and `course/PROGRESS.md`, and end with at most one focused reasoning question.
  - `next`: Follow the **Next exercise workflow** in `course/PLAYBOOK.md` — advance immediately without confirmation or explanation gates, preserve prior work and unverified outcomes honestly, scaffold one next exercise with edge-case tests, and reply briefly with the exercise link and scoped test command.
- **Coaching style:** Maintain one small, non-time-bound goal and a clear resumption point. Progress incrementally from fundamentals toward advanced topics. Explain Rust syntax and idioms with the learner's Java background in mind. Provide the smallest useful hint before complete solutions, and never rewrite or replace the learner's implementation unless explicitly asked.
