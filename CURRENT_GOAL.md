# Current Goal

## G024 - Minimum meeting rooms

**Status:** Ready to start; intentionally unfinished.
**Session:** Active.
**Course position:** Sorting and intervals (C5).
**Goal:** Implement `min_meeting_rooms(meetings: &[(i32, i32)]) -> usize`. No deadline.

## Next action

Read `touching_endpoints_share_a_room` in [the exercise](src/exercises/meeting_rooms.rs), then make an attempt when ready.

```powershell
cargo test --lib exercises::meeting_rooms::
```

## Contract

- Each tuple is a half-open meeting interval `[start, end)`; `start < end` is guaranteed. Input may be unsorted, duplicated, nested, or include negative and extreme `i32` timestamps.
- Endpoint policy: touching endpoints do NOT overlap. `(1, 3)` and `(3, 5)` share 1 room; `(1, 4)` and `(3, 5)` require 2 rooms.
- Return the minimum number of rooms needed so no overlapping meetings share a room (equivalently, the maximum concurrent meetings at any instant).
- Empty input returns `0`. Preserve borrowed input and support the full `i32` endpoint range.
- Safe Rust, `O(n log(n + 1))` time, `O(n)` auxiliary space. Standard sorting, copying input, and standard collections are allowed.
- Use an explicit loop for the sweep/counting pass; no coordinate enumeration, recursion, or calls to earlier exercises.

## Review criteria

- [ ] All 15 current tests pass.
- [ ] `rustfmt --edition 2024 --check src/exercises/meeting_rooms.rs` passes.
- [ ] `clippy-driver --edition=2024 --test src/exercises/meeting_rooms.rs --crate-name g024_review --emit=metadata --out-dir target -D warnings` passes.
- [ ] Explain state, invariant, touching-endpoint tie-breaking policy, branches, progress, and termination without case-specific patches.
- [ ] Explain arithmetic safety, borrowing, and time/auxiliary-space complexity.

## Resume notes

- Last completed action (2026-11-21): Scaffolded G024 on explicit `next` request. All 15 tests compile and fail at the intentional placeholder; 333 earlier tests filtered out. New-file formatting and standalone Clippy pass.
- Current blocker: None recorded; implementation intentionally unfinished.
- Next tiny action: Read `touching_endpoints_share_a_room` in `src/exercises/meeting_rooms.rs`.
- Prior learning: G023 last review passed 15/15 tests, `rustfmt`, and standalone Clippy; no correctness defect or demonstrated patchwork found. Sorted `start` ordering identified; full earlier-interval disjointness invariant and remaining reasoning pending. Earlier code and tests preserved. See [review](course/reviews/G023.md).

Say **start** / **resume**, **review**, **next**, or **stop**.

## Following exercise

Next: binary tree maximum depth with an explicit owned node representation (unit C6: Trees and heaps). Scaffold only on request.
