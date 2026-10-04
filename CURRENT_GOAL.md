# Current Goal

## G021 - Integer square root

**Status:** Ready to start; intentionally unfinished.
**Session:** Active.
**Course position:** Binary search and safe arithmetic.
**Goal:** Implement `integer_sqrt(n: u64) -> u64`. No deadline.

## Next action

Read `rounds_down_between_squares` in [the exercise](src/exercises/integer_sqrt.rs), then make an attempt when ready.

```powershell
cargo test --lib exercises::integer_sqrt::
```

## Contract

- Return the greatest integer r whose mathematical square is at most n (floor of the square root).
- Support every u64 input, including zero and u64::MAX, without overflow or panic.
- Use safe Rust and explicit loops; aim for O(log(n + 1)) time and O(1) auxiliary space.
- No floating point, built-in square-root/search helpers, recursion, linear scan, wider integer types, or calls to earlier exercises in the implementation. Checked integer arithmetic is allowed.
- Tests use u128 only to check mathematical results independently.

| Input | Result |
| --- | --- |
| 0 | 0 |
| 1 | 1 |
| 8 | 2 |
| 9 | 3 |
| 15 | 3 |
| u64::MAX | 4294967295 |

## Review criteria

- [ ] All 13 current tests pass.
- [ ] `rustfmt --edition 2024 --check src/exercises/integer_sqrt.rs` passes.
- [ ] `clippy-driver --edition=2024 --test src/exercises/integer_sqrt.rs --crate-name g021_review --emit=metadata --out-dir target -D warnings` passes.
- [ ] Explain state, invariant, branch decisions, progress, and termination without case-specific patches.
- [ ] Explain arithmetic safety and time/auxiliary-space complexity.

## Resume notes

- Last completed action (2026-10-04): Created G021 stub and 13 edge-case tests on explicit next request. All compile and fail at the intentional placeholder; 290 earlier tests filtered out. New-file formatting and standalone Clippy pass.
- Current blocker: None recorded; implementation intentionally unfinished.
- Next tiny action: Read `rounds_down_between_squares`.
- Prior learning: G020 last review passed 17/17 tests; formatting and two question_mark Clippy warnings remained. No correctness defect or demonstrated patchwork found. Learner described sorted-half selection correctly but confused midpoint-index equality with the value-equality return; coach supplied the distinction and a [5, 2], target 2 trace prompt. Follow-up and independent full reasoning remain unverified. Previous code and tests preserved. See [review](course/reviews/G020.md).

Say **review**, **next**, **stop**, or **resume**.

## Following exercise

Next: minimum feasible shipping capacity. Scaffold only on request.
