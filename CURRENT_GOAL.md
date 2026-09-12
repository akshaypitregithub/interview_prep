# Current Goal

## G019 - Lower bound

**Status:** Ready to start; intentionally unfinished.
**Session:** Active.
**Course position:** Binary search and boundaries.
**Goal:** Implement `lower_bound(nums: &[i32], target: i32) -> usize`. No deadline.

## Next action

Read `duplicates_in_middle_return_first` in [the exercise](src/exercises/lower_bound.rs), then make an attempt when ready.

```powershell
cargo test --lib exercises::lower_bound::
```

## Contract

- Input is guaranteed sorted in nondecreasing order. Do not sort or validate it with a pass.
- Return the first index whose value is at least target, or `nums.len()` if no value qualifies.
- With duplicates, the first qualifying position is required.
- Empty input returns 0. The result can equal the slice length; that is a position after its final element, not an index you can access.
- Support all i32 values and targets; preserve the borrowed slice and return positions relative to it.
- Use safe Rust and an explicit loop. No built-in search/partition helpers, linear scan, sorting, recursion, or calls to earlier exercises.
- Aim for O(log n) time for nonempty input and O(1) auxiliary space.

| Input / target | Result |
| --- | --- |
| `[1, 2, 2, 4]` / 2 | 1 |
| `[1, 2, 2, 4]` / 3 | 3 |
| `[1, 2, 2, 4]` / 5 | 4 |
| `[]` / 7 | 0 |

## Review criteria

- [ ] All 17 current tests pass.
- [ ] `rustfmt --edition 2024 --check src/exercises/lower_bound.rs` passes.
- [ ] `clippy-driver --edition=2024 --test src/exercises/lower_bound.rs --crate-name g019_review --emit=metadata --out-dir target -D warnings` passes.
- [ ] Explain the boundary invariant, branch decisions, strict progress and termination without case-specific patches.
- [ ] Explain first-position correctness, index safety and time/auxiliary-space complexity.

## Resume notes

- Last completed action (2026-09-10): G019 stub and 17 edge-case tests created; all compile and fail at the intentional placeholder. File formatted and standalone Clippy passes.
- Current blocker: None recorded; implementation intentionally unfinished.
- Next tiny action: Read `duplicates_in_middle_return_first`.
- Prior learning: G018 last review passed all 16 tests. Coach supplied the midpoint/progress explanation; independent reasoning remains unverified. Later cleanup observed but not rechecked. Earlier code preserved; see [review](course/reviews/G018.md).

Say **review**, **next**, **stop**, or **resume**.

## Following exercise

Next: search in a rotated sorted slice with distinct values. Scaffold only on request.
