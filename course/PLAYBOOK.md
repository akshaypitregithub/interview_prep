# Interview Practice Playbook

## Coding session

1. Restate the contract. Ask about size, ordering, duplicates, invalid input, mutation, numeric limits, and text representation when applicable.
2. Work a small example. Describe a correct baseline and its cost before optimizing.
3. State the invariant or recurrence and explain why the chosen structure fits the constraints.
4. Implement in small coherent steps while explaining decisions. Practice without autocomplete after basic Rust fluency is established.
5. Trace a normal case and a counterexample/boundary case. Fix defects and state time and auxiliary space, including recursion and allocations.
6. Handle a changed constraint. Record the failure mode and the smallest repair goal.

For practice tests, partition the contract into equivalence classes. Include empty/minimal inputs, boundaries, duplicates, absent results, extreme numbers, and adversarial order as applicable. For graphs include disconnected components and cycles; for strings choose explicit byte/character semantics; for arithmetic define overflow behavior. Add generated/property or brute-force comparison tests when the state space warrants them, without exposing the solution as a hint. No finite test suite establishes correctness for all inputs.

## Coaching protocol

Start with a question about the learner's reasoning. Escalate hints through: contract clarification → unrelated syntax example → conceptual nudge → counterexample → partial structure. Give a complete solution only when explicitly requested; label assisted work accurately and schedule independent retrieval. Do not solve G001 or quietly weaken its tests. If an attempt fails, identify one actionable issue before listing secondary improvements.

The learner owns the implementation and completion explanations. The coach scaffolds one exercise at a time, reviews evidence, and updates the active goal and progress after review. Never mark mastery solely because a solution was read or a generated answer compiled.

## Stop and resume workflow

Recognize standalone `stop` and `resume` commands case-insensitively, and equivalent explicit requests in context. Mentioning or defining the commands is not invoking them.

**Stop:** Stop exercise work promptly. Save a bookmark in `CURRENT_GOAL.md`: current goal ID and file, last observed implementation state, last completed action, blocker if known, and one concrete tiny next action. Mark the learning session paused without changing completion evidence, and record the pause in `course/PROGRESS.md`. Preserve learner code and tests. Do not run checks, review code, advance, give a solution, or ask another learning question. Use already known state, or a brief read of the current file if needed; label unverified state accurately. Finish with a short confirmation and the return bookmark. No deadlines, streaks, guilt, or “one more task.”

**Resume:** Read the saved bookmark and current exercise so external edits are respected. Restore the same goal and mark the session active in the goal and progress files. Give at most a sentence of context and one small action, such as reading a named test. No recap quiz, accumulated review queue, catch-up assignment, automatic test run, or requirement to complete the exercise. Continue from the saved point, without solving the learner's implementation. If no pause was saved, use the existing current goal; do not restart the course. The learner can stop again after that small action.

These commands control the learning session only; neither claims mastery nor resets progress. Their behavior overrides the usual coaching questions during the pause/resumption interaction. `review` and `next` retain their separate meanings.

## Repeatable implementation review

**Trigger:** Whenever the learner says "review" after implementing an exercise, review the implementation for the goal named in `CURRENT_GOAL.md`. Apply this workflow on every review, including follow-up attempts.

1. Read the current contract, implementation, tests, previous findings, and applicable repository instructions. Preserve the learner's code and tests; a review does not authorize rewriting their solution.
2. Review correctness first: contract compliance, edge cases, invariant/termination, panic and overflow risks, ownership/mutation behavior, and time/auxiliary-space complexity. Explain concrete defects with a failing case when possible. If no defect is found, say so explicitly and distinguish reasoning from test coverage.
3. Scope checks to the current exercise only. Run `cargo test --lib exercises::<module_name>::` (for G005: `cargo test --lib exercises::parse_integer::`), not the full suite or previous exercises. Check formatting with `rustfmt --edition 2024 --check src/exercises/<module_name>.rs`. Limit code and lint findings to the current exercise; do not make earlier exercises a completion gate. Cargo still compiles the shared library, and Cargo Clippy analyzes that crate rather than supporting a source-file filter: if using it, distinguish current-file diagnostics from unrelated diagnostics and do not describe a crate-wide exit code as a failure of the current exercise. Report a shared compilation blocker honestly without fixing or re-reviewing earlier exercises. Broaden review only on explicit user request. This scope overrides older whole-repository review criteria in course materials.
4. Review idiomatic Rust: borrowing and ownership, unnecessary clones/allocations, iterator versus indexing choices, pattern matching, `Option`/`Result`, return expressions, naming, imports, and obsolete scaffolding. Separate correctness defects, lint/format blockers, and optional preferences. Respect the exercise's learning constraints; do not blindly apply a lint suggestion that defeats them.
5. Present findings in priority order with file/line references, why each matters, and the smallest useful correction or hint. Avoid full replacement implementations unless explicitly requested. A valid index loop is not a correctness bug merely because an iterator would be more idiomatic.
6. Append a dated review entry to `course/reviews/<goal-id>.md`, preserving earlier attempts. Record checks, findings, help given, and remaining evidence. Update `CURRENT_GOAL.md` and `course/PROGRESS.md` with one exact next action.
7. During a review, keep the goal active until its implementation checks and the learner's own explanations are complete, unless the learner explicitly asks for `next`. End with at most one focused reasoning question when an explanation is still needed. Do not invent the learner's reasoning or silently advance to the next exercise.

### Patchwork assessment (required in every review)

Effective 2026-09-06, flag demonstrated patchwork as **invalid for exercise acceptance**, even if all tests pass. Assess whether the implementation has a coherent correctness argument: what its state means, what invariant it preserves, why each branch is valid, how remaining work decreases, and why termination establishes the result. Adapt this reasoning to the algorithm; do not demand loop terminology for a function without a loop.

Look for case-specific exceptions, arbitrary iteration budgets, compensating index adjustments, and exits that mask inconsistent state. These are investigation prompts, not automatic defects. Cite the exact condition and missing or broken assumption, with a counterexample when available. Give the smallest reasoning task needed to repair it, without supplying a replacement solution.

Record the assessment separately from test results: justified approach, demonstrated patchwork (invalid for acceptance), or reasoning unverified (acceptance pending). A passing suite does not prove a patch is valid. Conversely, do not claim a function is incorrect merely because the learner has not explained it: ask at most one focused question to resolve uncertainty. Debugger use, trial and error during development, legitimate boundary cases, verbosity, or deviation from a canonical solution do not themselves invalidate a solution. A correct brute-force algorithm is acceptable if it meets the contract and complexity requirements.

Require the learner's explanation before accepting a previously flagged approach; record assistance honestly. Explicit `next` still advances while preserving unresolved findings, and `stop`/`resume` retain their existing behavior.

## Next exercise workflow

**Trigger:** `next`, or an equivalent request to set up the next test. This explicit advancement request takes precedence over completion gates elsewhere in the course.

Record the previous goal's demonstrated results and any pending explanations without claiming mastery. Preserve its code and tests. Set up exactly one next exercise from the course sequence, with a clear contract, an unsolved Rust stub, and tests for applicable normal and boundary cases. Register the module, update `CURRENT_GOAL.md` and progress, and verify that the new tests compile and fail at the intentional placeholder. Keep previous tests active. Do not ask for confirmation, require an explanation before advancing, or provide the solution. Reply briefly with the exercise link and exact test command. `review` remains a separate request for critique.

For G001, the required explicit loop conflicts with Clippy's `manual_find` suggestion on the initial attempt. Its goal-specific command allows only that lint: `cargo clippy --all-targets -- -D warnings -A clippy::manual_find`. All other warnings remain errors. This is an instructional exception, not permission to suppress unrelated findings or use a search helper.

For G004, the learner explicitly chose to keep the manual swap while learning. Accept this correct approach and do not require `slice::swap`. When checking all targets including G004, use `cargo clippy --all-targets -- -D warnings -A clippy::manual_find -A clippy::manual_swap`. The two exceptions apply to these teaching choices; other warnings remain errors. Carry this command into later goals while the earlier manual-swap exercise remains in the package.

## System design outline

Use this as a discussion structure, not a memorized architecture:

- **Problem:** users, core operations, exclusions, consistency needs, latency/availability objectives, and assumptions.
- **Scale:** average/peak request rate, payload sizes, retained data, bandwidth; show units and sensitivity to assumptions.
- **Interfaces and data:** example request/response, IDs, schema, access patterns, authorization, and idempotency boundaries.
- **Initial design:** request path, source of truth, components, and one alternative considered.
- **Deep dive:** the most consequential bottleneck or correctness risk; show behavior during timeout, overload, duplicate delivery, or partition.
- **Operations and evolution:** metrics/alerts, recovery, security/privacy, cost, staged migration, rollback, ownership, and unresolved risks.

Save each design under `course/designs/` when activated. Use a small diagram only when it clarifies the data flow. Defend choices using constraints; naming technologies is not a substitute for explaining behavior.

## Behavioral story template

Create individual notes under `course/stories/` when activated. Use this outline:

```text
Story / applicable prompts:
Situation and stakes:
My responsibility and scope:
Constraints and stakeholder disagreement:
Alternatives considered and why I chose one:
My specific actions (separate from the team's):
Result, metric, and how it was measured:
What failed or remained unresolved:
What I learned / would change:
Follow-up questions I need to prepare:
```

Build coverage for ambiguous requirements, influence without authority, mentoring, disagreement, failure, incidents, customer impact, and delivery tradeoffs. Use real examples and accurate attribution. If a metric is unknown, say so and explain the qualitative evidence. Anonymize confidential details before putting them in this repository.

## Mock interviews

Learning is untimed. When ready, explicitly choose a simulation with the recruiter's round length, environment, and tooling policy; if unknown, a 45-minute coding or design session is a practice assumption, not a company rule. Include clarification, execution, testing, and follow-ups within that session.

A default full mock loop contains two coding sessions, one design session, and one leadership/project deep dive. Adapt to the confirmed role format. Mix unseen prompts, rotate topics, and record whether the interviewer offered a meaningful hint. Practice both unaided work and tool-assisted work only when the target interview allows it.

After each mock, record a short evidence-based assessment:

| Score | Observable behavior |
| --- | --- |
| 0 — Not demonstrated | Cannot produce or explain a viable approach. |
| 1 — Guided | Substantial hints or corrections are required. |
| 2 — Independent with gaps | Main approach works independently, but important testing, reasoning, or tradeoff gaps remain. |
| 3 — Interview-ready evidence | Independent, correct, clear, handles relevant follow-ups, and meets the criteria below. |

Score coding correctness, algorithm selection/complexity, language fluency, design judgment, senior ownership, and communication separately where observed. Do not average a critical failure away. Passing means evidence on that prompt, not a hiring prediction.

## Readiness criteria

These are course-defined thresholds, not published company cutoffs:

- **Coding:** At least 8 of the last 10 unfamiliar mixed prompts completed independently with correct reasoning and boundary handling, across at least five families; score 3 for correctness and algorithm reasoning. Re-solve three earlier weaknesses after intervening goals without notes.
- **Rust:** Can write and debug common collection, traversal, and error-handling code in the permitted environment without persistent ownership or syntax blockage. Verify Rust is accepted for the target round.
- **Design:** Three distinct design discussions at score 3, covering requirements, data, scale, a deep dive, failure behavior, tradeoffs, and evolution. Defend a changed requirement without restarting from a memorized template.
- **Leadership:** 8–10 factual story outlines with coverage across the listed themes; two deep dives withstand probing about individual contribution, alternatives, outcomes, and reflection. Obtain an independent review where possible.
- **Mock loops:** Two complete loops on different prompts with score 3 in every observed core dimension. A gap triggers focused remediation and a fresh assessment, not repeating the same memorized prompt.
- **Role fit:** Resume claims, real project scope, specialty knowledge, and recruiter-confirmed format are aligned. Prepare thoughtful questions about team challenges, decision ownership, and success at the level.

Use [progress](PROGRESS.md) to preserve the evidence. Once thresholds are met, maintain a small retrieval goal and address role-specific gaps rather than endlessly expanding the problem list.
