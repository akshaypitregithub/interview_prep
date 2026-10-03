# Current Goal

## G020 - Search rotated sorted input

**Status:** Ready to start; intentionally unfinished.
**Session:** Active.
**Course position:** Binary search and boundaries.
**Goal:** Implement `rotated_search(nums: &[i32], target: i32) -> Option<usize>`. No deadline.

## Next action

Read `finds_targets_on_both_sides_of_rotation` in [the exercise](src/exercises/rotated_search.rs), then make an attempt when ready.

```powershell
cargo test --lib exercises::rotated_search::
```

## Contract

- Input is a rotation of a strictly increasing sequence; all values are distinct. Empty and unrotated inputs are valid. Do not validate or sort it.
- A rotation moves a prefix to the end without changing order: `[2, 4, 6, 8, 10]` can become `[8, 10, 2, 4, 6]`.
- Return `Some(index)` for a present target, otherwise `None`. Indices are relative to the supplied slice.
- Preserve the borrowed slice and support every i32 value and target.
- Use safe Rust and explicit loops. No linear scan, sorting, recursion, built-in search/partition helpers, or calls to earlier exercises.
- Aim for O(log n) time for nonempty input and O(1) auxiliary space.

| Input / target | Result |
| --- | --- |
| `[8, 10, 2, 4, 6]` / 2 | `Some(2)` |
| `[8, 10, 2, 4, 6]` / 9 | `None` |
| `[2, 4, 6]` / 6 | `Some(2)` |
| `[]` / 7 | `None` |

## Review criteria

- [ ] All 17 current tests pass.
- [ ] `rustfmt --edition 2024 --check src/exercises/rotated_search.rs` passes on the implementation.
- [ ] `clippy-driver --edition=2024 --test src/exercises/rotated_search.rs --crate-name g020_review --emit=metadata --out-dir target -D warnings` passes on the implementation.
- [ ] Explain the candidate invariant, branch decisions, strict progress and termination without case-specific patches.
- [ ] Explain index safety and time/auxiliary-space complexity.

## Resume notes

- Last completed action (2026-09-21): G020 stub and 17 edge-case tests created. All compile and fail at the intentional placeholder; 273 earlier tests filtered out. New file formatting and standalone Clippy pass.
- Current blocker: None recorded; implementation intentionally unfinished.
- Next tiny action: Read `finds_targets_on_both_sides_of_rotation`.
- Prior learning: G019 last review passed all 17 tests and Clippy. Formatting cleanup now observed but not rechecked. Coach supplied the precise candidate invariant and branch justification after the learner said their answer was a guess; independent reasoning remains unverified. See [review](course/reviews/G019.md). Earlier code preserved.

Say **review**, **next**, **stop**, or **resume**.

## Following exercise

Next: integer square root with safe arithmetic. Scaffold only on request.
