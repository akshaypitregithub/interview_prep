# Repository Guidelines

## Project Structure & Module Organization

This repository supports incremental data structures and algorithms (DSA) interview preparation in Rust. It currently contains a single binary entry point, `src/main.rs`, with a Hello World program. `Cargo.toml` defines the Rust 2024 package and currently has no dependencies; `Cargo.lock` records dependency resolution. `note_for_codex` describes learning preferences. Generated build output belongs in `target/`, which Git ignores. No separate tests or assets directories exist yet.

As exercises grow, organize them into focused modules under `src/`, using descriptive names such as `binary_search.rs`. Declare new modules from the appropriate parent module so Cargo compiles and tests them.

## Build, Test, and Development Commands

Run these commands from the repository root with a Rust toolchain supporting edition 2024:

- `cargo build`: compile the project in debug mode.
- `cargo run`: execute the current binary.
- `cargo test`: run the test suite as exercises are added.
- `cargo test binary_search`: run tests matching a name filter.
- `cargo fmt --check`: check formatting with rustfmt; use `cargo fmt` to apply it.
- `cargo clippy --all-targets -- -D warnings`: check all targets for lint warnings.

## Coding Style & Naming Conventions

Use standard rustfmt formatting and four-space indentation. Use `snake_case` for functions, modules, and variables; `PascalCase` for types and traits; and `SCREAMING_SNAKE_CASE` for constants. Prefer small functions with explicit inputs and outputs. Explain algorithm invariants and time/space complexity where useful. No custom formatter or lint configuration is currently present.

## Testing Guidelines

Use Rust's built-in `#[test]` framework, placing unit tests in a `#[cfg(test)] mod tests` block alongside each exercise. Name tests by behavior, such as `returns_none_when_target_is_absent`. Cover normal cases and applicable boundaries: empty inputs, single elements, duplicates, missing values, and numeric limits. No coverage threshold is configured. Run tests before submitting changes.

## Commit & Pull Request Guidelines

There are no commits yet, so no established message convention exists. Use concise, imperative messages such as `Add binary search exercise`. Keep changes focused. Pull requests should describe the exercise or change, relevant complexity tradeoffs, and validation performed; link an issue when applicable.

## Learning & Agent Instructions

Follow `note_for_codex`: maintain one small, non-time-bound goal and a clear resumption point. Progress from fundamentals toward advanced topics. Explain Rust syntax and idioms with the learner's Java background in mind. Provide hints before complete solutions, and scaffold exercises with edge-case tests.
