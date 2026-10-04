# Current Goal

## G022 - Minimum shipping capacity

**Status:** Ready to start; intentionally unfinished.
**Session:** Active.
**Course position:** Binary search on a feasible answer.
**Goal:** Implement `shipping_capacity(weights: &[u32], days: usize) -> Option<u64>`. No deadline.

## Next action

Read `finds_minimum_for_multiple_days` in [the exercise](src/exercises/shipping_capacity.rs), then make an attempt when ready.

```powershell
cargo test --lib exercises::shipping_capacity::
```

## Contract

- Find the smallest daily capacity that ships all packages within the given number of days.
- Preserve order; packages cannot be split. Each used day ships a contiguous group of remaining packages, with total weight at most the capacity. Unused days are allowed.
- Zero weights are valid. Empty input returns Some(0), even with zero days. Nonempty input with zero days returns None, including all-zero weights.
- Otherwise return Some(capacity). The mathematical sum of weights is guaranteed to fit u64. Preserve the borrowed input.
- Use safe Rust and explicit loops, O(n * log(S + 2)) time and O(1) auxiliary space, where n is package count and S is total weight.
- No sorting, recursion, floating point, scanning every possible capacity, built-in search/partition helpers, or calls to earlier exercises.

| Weights / days | Result |
| --- | --- |
| [3, 2, 2, 4, 1, 4] / 3 | Some(6) |
| [1, 10, 1] / 2 | Some(11) |
| [1, 10, 1] / 3 | Some(10) |
| [0, 0] / 1 | Some(0) |
| [1] / 0 | None |
| [] / 0 | Some(0) |

## Review criteria

- [ ] All 15 current tests pass.
- [ ] `rustfmt --edition 2024 --check src/exercises/shipping_capacity.rs` passes.
- [ ] `clippy-driver --edition=2024 --test src/exercises/shipping_capacity.rs --crate-name g022_review --emit=metadata --out-dir target -D warnings` passes.
- [ ] Explain feasibility, candidate bounds, invariant, branches, progress, and termination without case-specific patches.
- [ ] Explain arithmetic safety and time/auxiliary-space complexity.

## Resume notes

- Last completed action (2026-10-04): Created G022 stub and 15 tests on explicit next request. All compile and fail at the intentional placeholder; 303 earlier tests filtered out. New-file formatting and standalone Clippy pass.
- Current blocker: None recorded; implementation intentionally unfinished.
- Next tiny action: Read `finds_minimum_for_multiple_days`.
- Prior learning: G021 last review passed all 13 tests, formatting, and Clippy. No correctness defect or demonstrated patchwork found. Learner justified rejecting an overflowing midpoint because n is bounded by u64; justification for discarding all larger candidates and remaining independent reasoning are pending. Earlier code and tests preserved. See [review](course/reviews/G021.md).

Say **review**, **next**, **stop**, or **resume**.

## Following exercise

Next: merge intervals with an explicit touching-endpoint policy. Scaffold only on request.
