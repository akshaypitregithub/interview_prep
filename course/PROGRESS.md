# Progress and Review Queue

The authoritative next action lives in [CURRENT_GOAL.md](../CURRENT_GOAL.md). There are no deadlines, streaks, or automatic catch-up assignments.

## Starting point

- Language: Rust; prior background: Java; Rust refresher requested.
- Intended level: Senior software engineer.
- Specialty: Backend/distributed systems assumed until clarified.
- Experience, previous interview feedback, and strongest/weakest areas: Not yet assessed.
- Baseline: Original Hello World package compiled and had zero tests. G001 is scaffolded and intentionally unsolved.
- Assessment policy: Do not infer skill from years of experience or from a scaffold passing build checks.

## Setup verification

On 2026-09-05, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo run` passed with Rust/Cargo 1.98.1. `cargo test first_index` compiled and ran all 12 tests; all failed at the intentional `todo!` placeholder. This verifies the exercise is connected to Cargo, not that an implementation is correct. G001 remains unattempted.

## Active goal

**Session active.** Advanced to G022 minimum shipping capacity on explicit next request (2026-10-04). All 15 new tests compile and fail at the intentional placeholder; 303 earlier tests filtered out. New-file formatting and standalone Clippy pass. Next tiny action: read `finds_minimum_for_multiple_days` in `src/exercises/shipping_capacity.rs`. Earlier implementations and tests preserved.

G021 advancement evidence: last review passed all 13 tests, file formatting, and standalone Clippy. No correctness defect or demonstrated patchwork found. Learner justified rejecting an overflowing midpoint using the u64 bound on n; the argument for rejecting all larger candidates and other independent reasoning remain pending. Coach clarified the mathematical-square interpretation and asked how the square changes as the nonnegative candidate increases; that follow-up remains unanswered. No G021 checks rerun during advancement. See [review](reviews/G021.md).

G020 advancement evidence: last review passed 17/17 tests; formatting and two question_mark Clippy warnings remained. No correctness defect or demonstrated patchwork found. Learner correctly described selecting by a sorted half's range, but confused midpoint-index equality with the value-equality early return. Coach clarified the distinction and supplied a [5, 2], target 2 trace prompt; follow-up remains unanswered. Independent full correctness, progress, safety, and complexity reasoning remains unverified. See [review](reviews/G020.md).

G019 advancement evidence: last review passed all 17 tests and standalone Clippy, with no correctness defect or demonstrated patchwork. Formatting cleanup now observed but not rechecked. Coach supplied the invariant and branch justification after learner clarified their answer was a guess. Independent correctness/progress/complexity reasoning remains unverified. See [review](reviews/G019.md).

G018 advancement evidence: last review passed all 16 tests with no correctness defect or demonstrated patchwork. The coach subsequently supplied the inclusive-midpoint progress explanation; independent invariant/complexity reasoning remains unverified. Removal of `mut` and formatting cleanup observed on advancement but not rechecked. See [review](reviews/G018.md).

| ID | Unit | Deliverable | Status |
| --- | --- | --- | --- |
| G022 | Minimum shipping capacity | Smallest capacity to ship in order within a day limit | Unsolved scaffold; 15 tests |

Setup evidence (not rerun on resume): only G006's 13 tests ran; each failed at the intentional placeholder, with 62 earlier tests filtered out. G006 file formatting and standalone Clippy passed. Previous implementations were preserved. The checkpoint paragraphs below are historical.

Latest review, G005 attempt 1 (2026-09-05): only its 13 tests ran and passed; standalone Clippy passed. Formatting fails on two blank lines. Directly returning the parser's Result is correct and idiomatic. See [review](reviews/G005.md). Current next action: format `src/exercises/parse_integer.rs` only. No source or tests modified; learner explanation remains pending. Setup notes below are historical.

Current next action: run `cargo test parse_integer`. G005 setup verification: all 49 earlier tests, formatting, and Clippy with the two documented teaching exceptions pass. All 13 new tests compile and fail at the intentional placeholder. Earlier code and tests remain unchanged. The entries below describe earlier checkpoints, not pending actions for G005.

Advanced to G004 on explicit `next` request. G003's formatting is now fixed. Setup verification: all 36 earlier tests, repository formatting, and documented Clippy pass; all 13 new tests compile and fail at the intentional `todo!` placeholder. Next action: run `cargo test reverse_in_place`. Earlier implementations and tests are preserved.

Latest G004 review, attempt 1 (2026-09-05): all 49 tests pass; no correctness defect found. Formatting fails on line 10; Clippy reports `manual_swap`. See [review](reviews/G004.md). Current next action: replace the manual swap with the permitted slice method, then format and rerun checks. No source or tests rewritten; learner explanations remain pending.

Learner clarification supersedes the swap recommendation: keep the manual swap and allow `clippy::manual_swap` alongside G001's `manual_find` exception. The learner correctly explained that revisiting mirrored pairs undoes reversal. Current next action: run `cargo fmt` and the updated lint command in `CURRENT_GOAL.md`; no source changes or new verification performed in this clarification.

G003 setup verification: all 23 earlier tests pass; formatting and Clippy with the documented G001 exception pass. All 13 G003 tests compile and fail at the intentional `todo!` placeholder. No solution supplied.

## Previous goals

Record implementation completion separately from unverified learning outcomes. Explicit `next` requests advance even when explanations remain pending.

| ID / topic | Evidence or file | Help used (none/syntax/concept/algorithm/solution) | Correctness and reasoning | Follow-up |
| --- | --- | --- | --- | --- |
| G001 / first matching index | [Implementation and review](reviews/G001.md) | Rust syntax/idiom feedback; no replacement solution | Implementation checks pass; learner correctly explained borrowed `i32` and dereferencing | Revisit first-match/empty-case reasoning, `Option<usize>`, and complexity; not yet independently explained |
| G002 / count occurrences | [Review](reviews/G002.md) | Focused loop-header feedback; ownership/iteration explanations | Implementation checks pass; full-scan and constant-space reasoning accepted | Retrieve asymptotic time notation and borrowing versus consuming |
| G003 / maximum value | [Review](reviews/G003.md) | Pattern-matching and initialization hints; no replacement function | Single if-let implementation passes tests, formatting, and Clippy | Empty/all-negative reasoning and complexity explanations remain unverified |
| G004 / reverse in place | [Review](reviews/G004.md) | Manual swap retained by preference; value/reference and ownership explanations | All 49 earlier tests, formatting, and revised Clippy pass; midpoint reasoning accepted | Retrieve bounds, mutable borrowing, complexity, and value versus reference storage |
| G005 / parse integer | [Review](reviews/G005.md) | Result/caller-handling explanation; no replacement function | Last review: 13 tests and standalone Clippy pass; learner identified error payload in Err | Two formatting blank lines remain; retrieve caller error handling and why not to panic/default |
| G006 / running totals | [Review](reviews/G006.md) | Vec initialization/capacity hints and space explanation | Last review: all 13 tests, formatting, standalone Clippy pass | Prefix/overflow reasoning and output versus auxiliary space not independently explained |
| G007 / sorted deduplication | [Review](reviews/G007.md) | Range syntax and initialization hints; invariant correction | Last review: 13 tests, formatting, standalone Clippy pass | Learner related count to iterations; corrected strict inequality to count <= i at writes; retrieve full invariant and complexity |
| G008 / sorted merge | [Review](reviews/G008.md) | Allocation and loop critique; no replacement function | Last review: 13 tests and formatting pass; allocation issue resolved | Last lint finding: outer if parentheses; not reverified on advancement. Merge reasoning and complexity explanations remain unverified |

| G009 / ASCII palindrome | [Review](reviews/G009.md) | Coach supplied invariant/algorithm framing; learner wrote rewrite | Last review: 13 tests and counterexample pass; coherent algorithm; learner identified empty and loop guards preventing underflow | Later lint/format edits observed but not rechecked on advancement; retrieve full invariant, text semantics and complexity |

| G010 / character frequencies | [Review](reviews/G010.md) | HashMap/Unicode syntax help; invariant refined and formal proof explained by coach | Last review: all 13 tests, formatting and Clippy pass; learner described counts for processed input | Retrieve per-key invariant proof, Unicode and expected complexity independently; obsolete placeholder statement remains optional cleanup |

| G011 / first unique character | [Review](reviews/G011.md) | Permanent-repeat state and traversal hints; complexity and linked-list tradeoff discussion | Last review: 16 tests, two counterexamples, formatting and Clippy pass; learner identified unordered map traversal needs a minimum-position candidate | Retrieve invariants and expected time/space independently; no linked-list implementation or mastery claimed |

| G012 / Unicode anagrams | [Review](reviews/G012.md) | Struct/map syntax assistance; paired-prefix proof supplied by coach | Last review: 17 tests pass; learner correctly related equal scalar frequencies to equal byte length | Later guard/counter cleanup observed but not rechecked; retrieve paired traversal proof and complexity independently |

| G013 / two sum | [Review](reviews/G013.md) | Complement-lookup hint; checked arithmetic and complexity explanation | Last review: 16 tests, overflow counterexample and Clippy pass; learner identified quadratic baseline and implemented linear alternative | Formatting last failed; no recheck on advancement. Retrieve lookup-before-insert invariant and independent complexity comparison |

| G014 / ordered intersection | [Review](reviews/G014.md) | Membership idiom and phase-structure feedback | Last review: 14 tests pass; no correctness defect or patchwork found | Formatting and membership lint last failed; not rechecked. Retrieve flag invariant, phase order and complexity |

| G015 / longest consecutive | [Review](reviews/G015.md) | Learner read approach; coach explained boundary ownership and aggregate analysis; borrow/dereference syntax help | Last review: 17 tests, formatting and Clippy pass; large-run counter overflow identified statically | Counter fix not reverified; independently re-derive start-only traversal and total-work proof later |

| G016 / maximum window sum | [Review](reviews/G016.md) | Overlapping-window hint and complexity explanation | Last review: 16 tests and Clippy pass; correct O(n) overlap-sum reuse | Formatting last failed; independent overlap invariant and width-one reasoning pending |

| G017 / longest unique substring | [Review](reviews/G017.md) | Unicode and two-iterator hints; synchronization feedback | Last review: 17 tests, counterexample, formatting and Clippy pass; learner explained consuming prior occurrence and retaining current set entry, refined to current-window membership | Retrieve complete window/completeness invariant and aggregate work independently |

## Retrieval queue

Revisit earlier topics after intervening exercises, without blocking explicit `next` requests. A forgotten topic becomes a smaller repair goal, not a failed streak.

| Topic | Trigger (completed-goal count) | Retrieval result | Next review |
| --- | --- | --- | --- |
| G001 reasoning and types | Due: G002 and G003 implemented; revisit when requested, without blocking next | Borrow/dereference explanation accepted; remaining explanations pending | First-match behavior, empty input, `Option`, complexity |
| G002 counting and iteration | Due: G003 and G004 implemented; does not block next | Full scan and constant extra space explained correctly | Time notation; array/slice and borrowing/consuming distinctions |
| G003 optional accumulator | After two intervening exercises are implemented | Correct implementation; explanations pending | Empty/all-negative cases, complexity; implicit iteration and compile/runtime costs discussed with coach |
| G004 reversal and storage | After two intervening exercises are implemented | Correct midpoint explanation; other topics discussed with coach | Safe indices, mutable borrow, complexity, Copy and reference elements |
| G005 Result and parsing | After two intervening exercises are implemented | Learner correctly identified Err as containing the error; caller handling explained by coach | Error propagation versus handling; formatting cleanup |
| G006 prefix sums and capacity | After two intervening exercises are implemented | Implementation correct; early-error capacity reasoning explained by coach | Output versus auxiliary space, capacity versus length, first overflow |
| G007 compaction invariant | After two intervening exercises are implemented | Partially correct explanation; equality case and distinct-prefix invariant clarified by coach | Write index <= read index, sortedness, logical length, complexity |
| G008 sorted merge | After two intervening exercises are implemented | Correct output; allocation fix reviewed | Sorted output invariant, preserving duplicates, remainders, space and lint follow-up |

## Evidence dashboard

| Dimension | Current evidence | Next assessment |
| --- | --- | --- |
| Rust | G004 checks pass with accepted manual swap; value/reference storage and Copy discussed | Practice Result and borrowed text in G005 |
| DSA / complexity | G002 full-scan and constant-space explanations accepted | State asymptotic time during later recall; reason about G003 |
| Systems / design | Not assessed | S0 request trace after C1 |
| Leadership / scope | Not assessed | L0 real-project inventory after C1 |
| Communication | Correct explanation of G002 full scan and G001 dereferencing | Explain complexity concisely |
| Target role / format | Not confirmed | Recruiter/job-description calibration |

## Mock log

No mocks attempted. Record prompt, novelty, environment/tool rules, hints, per-dimension scores from the [playbook](PLAYBOOK.md), concrete evidence, and the next repair goal. Keep learning completion and mock readiness separate.

## Resume rule

### 2026-10-03 — G020 resumed

Session active; continuing rotated sorted search. An implementation attempt is present and preserved; no review or checks run on resume. No blocker recorded. Next tiny action: read `finds_targets_on_both_sides_of_rotation` in `src/exercises/rotated_search.rs`.

### 2026-09-09 — G018 resumed

Session active; continuing binary search. An implementation attempt is present and preserved; no review or tests run on resume. No blocker recorded. Next tiny action: read `singleton_absent_on_either_side` in `src/exercises/binary_search.rs`.

Before stopping, update the current goal with the last completed action, blocker, and exact next tiny action. On `next`, archive current evidence here and activate exactly one next exercise without confirmation or explanation gates. Preserve pending learning outcomes honestly; progression does not itself establish mastery.
