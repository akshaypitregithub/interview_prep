# Current Goal

## G023 - Merge intervals

**Status:** Ready to start; intentionally unfinished.
**Session:** Active.
**Course position:** Sorting and intervals.
**Goal:** Implement `merge_intervals(intervals: &[(i32, i32)]) -> Vec<(i32, i32)>`. No deadline.

## Next action

Read `merges_unsorted_overlaps` in [the exercise](src/exercises/merge_intervals.rs), then make an attempt when ready.

```powershell
cargo test --lib exercises::merge_intervals::
```

## Contract

- Each tuple is a closed interval; start <= end is guaranteed. Input may be unsorted, duplicated, nested, negative, or zero-length.
- Return the union sorted by start, merging overlaps and shared endpoints. Consecutive output intervals satisfy previous.end < next.start.
- (1, 3) and (3, 5) merge to (1, 5); (1, 2) and (3, 4) stay separate.
- Empty input returns an empty Vec. Preserve borrowed input and support the full i32 endpoint range.
- Safe Rust, O(n log(n + 1)) time, O(n) auxiliary space. Standard sorting and copying input are allowed.
- Use an explicit merging loop; no coordinate enumeration, recursion, or calls to earlier exercises.

## Review criteria

- [ ] All 15 current tests pass.
- [ ] `rustfmt --edition 2024 --check src/exercises/merge_intervals.rs` passes.
- [ ] `clippy-driver --edition=2024 --test src/exercises/merge_intervals.rs --crate-name g023_review --emit=metadata --out-dir target -D warnings` passes.
- [ ] Explain state, invariant, overlap policy, branches, progress, and termination without case-specific patches.
- [ ] Explain arithmetic safety, borrowing, and time/auxiliary-space complexity.

## Resume notes

- Last completed action (2026-10-07): Scaffolded G023 on explicit next request. All 15 tests compile and fail at the intentional placeholder; 318 earlier tests filtered out. New-file formatting and standalone Clippy pass.
- Current blocker: None recorded; implementation intentionally unfinished.
- Next tiny action: Read `merges_unsorted_overlaps`.
- Prior learning: G022 last review passed 15/15 tests and standalone Clippy; one formatting space remained. Monotonicity verified; greedy optimality and remaining reasoning pending. No functional correctness defect or demonstrated patchwork found. Earlier code and tests preserved. See [review](course/reviews/G022.md).

Say **review**, **next**, **stop**, or **resume**.

## Following exercise

Next: meeting-room count with an explicit endpoint policy. Scaffold only on request.
