# Current Goal

## G025 - Binary tree maximum depth

**Status:** Ready to start; intentionally unfinished.
**Session:** Active.
**Course position:** Trees and heaps (C6).
**Goal:** Implement `max_depth(root: Option<&TreeNode>) -> usize`. No deadline.

## Next action

Read `TreeNode` and `balanced_complete_tree` in [the exercise](src/exercises/tree_depth.rs), then make an attempt when ready.

```powershell
cargo test --lib exercises::tree_depth::
```

## Contract

- `TreeNode` owns optional children via `Option<Box<TreeNode>>`; `max_depth` receives a borrowed root `Option<&TreeNode>`.
- Depth is measured in **nodes** along the longest root-to-leaf path: `None` returns `0`, a single leaf returns `1`, and a root with one child returns `2`.
- Node values (`i32`) may be negative, duplicated, or extreme (`i32::MIN`, `i32::MAX`) and do not affect depth.
- Preserve the borrowed tree (`Option<&TreeNode>`).
- Safe Rust, `O(n)` time, `O(h)` auxiliary space (where `h` is the tree height, up to `O(n)` on skewed trees).
- Either recursive or iterative traversal is allowed; no calls to earlier exercises.

## Review criteria

- [ ] All 13 current tests pass.
- [ ] `rustfmt --edition 2024 --check src/exercises/tree_depth.rs` passes.
- [ ] `clippy-driver --edition=2024 --test src/exercises/tree_depth.rs --crate-name g025_review --emit=metadata --out-dir target -D warnings` passes.
- [ ] Explain base case, recursive/iterative state, invariant, progress, and termination without case-specific patches.
- [ ] Explain `Option<Box<TreeNode>>` borrowing, call-stack/auxiliary space on balanced vs. skewed trees, and time complexity.

## Resume notes

- Last completed action (2026-11-27): Scaffolded G025 on explicit `next` request. All 13 tests compile and fail at the intentional placeholder; 348 earlier tests filtered out. New-file formatting and standalone Clippy pass.
- Current blocker: None recorded; implementation intentionally unfinished.
- Next tiny action: Read `TreeNode` and `balanced_complete_tree` in `src/exercises/tree_depth.rs`.
- Prior learning: G024 last review passed 15/15 tests, `rustfmt`, and standalone Clippy; no correctness defect or demonstrated patchwork found. Half-open tie-breaking, `O(n log n)` time, and `O(n)` auxiliary space verified; arithmetic safety, borrowing, and independent `start < end` prefix recall pending. Earlier code and tests preserved. See [review](course/reviews/G024.md).

Say **start** / **resume**, **review**, **next**, or **stop**.

## Following exercise

Next: binary tree level-order traversal (BFS with `VecDeque`, unit C6: Trees and heaps). Scaffold only on request.
