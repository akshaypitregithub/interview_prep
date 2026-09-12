# Senior Engineer Interview Course

## Target and scope

Prepare to demonstrate senior-level problem solving, sound architecture, production judgment, and influence across a team or teams. Coding uses Rust, starting from rusty fundamentals and a Java background. Backend/distributed systems is the provisional specialization. There is no fixed schedule and no promise that course completion guarantees an offer: company, team, level, and hiring decisions vary.

The curriculum below is our preparation plan, not an official company syllabus. See [sources](SOURCES.md) for the evidence behind the broad interview areas and the limits of company-specific information.

## How the course runs

Say **next** to activate the next exercise and its tests immediately. Pending explanations are recorded for later review and do not block this explicit request. Say **review** separately for implementation critique. Advancement does not automatically certify mastery.

1. Read only the current goal. Each goal produces one small artifact: a function, a design decision, a story outline, or a mock review.
2. Attempt before asking for a solution. Use the smallest needed hint; log whether help was syntax, concept, or algorithm related.
3. Review correctness, reasoning, communication, and applicable edge cases. Passing tests alone is insufficient.
4. Save an exact next action whenever you stop. Break large exercises into contract, baseline, optimization, and explanation goals.
5. After every two new goals, retrieve one earlier topic without notes. Revisit again after roughly six intervening goals; failed recall creates a smaller repair goal. These are activity counts, not deadlines.

Use the coding sequence in order. After C1, interleave one systems or leadership goal after every two coding goals; choose from whichever track needs evidence most. This is a default, not a quota. Advance tracks independently so leadership preparation does not wait for advanced DSA.

Each coding unit follows **learn → representative exercise → contrasting variant → unfamiliar check → later recall**. Named exercises below describe problem families, not copied statements; the coach writes an explicit contract and edge-case tests when activating each one. Future exercises are a curriculum backlog, not runnable assignments yet.

## Coding and Rust sequence

### C0 — Rust re-entry

Learn function signatures, bindings and `mut`, expressions, loops, slices, `usize`, `Option`, and reading compiler/test output. Then learn ownership, borrowing, `Vec`, and `Result` as needed.

Practice in order: **first matching index (G001)**; count occurrences; maximum of an empty-or-nonempty slice; reverse a mutable slice; parse a number with an explicit error result. Each is a separate goal. Keep unrelated Rust features out of the first task.

**Gate:** Implement two small functions independently, explain borrow versus ownership, and handle empty input without a magic sentinel or unnecessary clone.

### C1 — Complexity, arrays, and strings

Learn asymptotic versus measured cost, best/worst/amortized behavior, loop invariants, allocation costs, and overflow. Distinguish UTF-8 bytes, Unicode scalar values, and user-perceived characters before choosing a string representation.

Practice: running totals with a defined overflow policy; sorted deduplication in place; merge sorted arrays; ASCII palindrome; character frequencies with an explicit Unicode contract. Compare a quadratic baseline with a linear alternative.

**Gate:** Derive time and auxiliary space from the operations used, explain an invariant, and create adversarial tests before coding. Then activate the first S0 or L0 goal.

### C2 — Hashing and prefix sums

Learn `HashMap`, `HashSet`, `entry`, hashing assumptions, frequency tables, and prefix-state reasoning.

Practice: duplicate detection; pair-sum indices; grouping anagrams under an ASCII contract; range-sum queries; number of subarrays with a target sum. Include repeated values, negative values, and wider sum types where appropriate.

**Gate:** Explain expected hash-map complexity, account for stored state, and solve a new lookup/counting variant without being told the pattern.

### C3 — Two pointers and sliding windows

Learn pointer invariants, fixed versus variable windows, and when shrinking is valid.

Practice: pair sum in sorted input; three-value sum with deduplication; maximum fixed-window sum; longest substring without repeated characters; minimum covering window. Contrast positive-only window sums with negative-number inputs that break the reasoning.

**Gate:** Justify every pointer move and why the total work is linear when claimed. Explain when a window method does not apply.

### C4 — Stacks, queues, and linked structures

Learn `Vec` as a stack, `VecDeque`, monotonic stacks, node ownership, `Box`, and `Option::take`. Understand shared ownership only when a representation requires it; do not introduce `unsafe` to bypass a borrow error.

Practice: balanced delimiters; minimum stack; next greater element; queue from two stacks; reverse an owned singly linked list; merge sorted lists; cycle reasoning with an explicitly chosen index-based or shared-node representation.

**Gate:** Explain ownership and amortized costs, and trace empty, single-node, and duplicate cases. Separate the algorithm from Rust representation difficulties.

### C5 — Sorting, binary search, and intervals

Learn comparator contracts, stable versus unstable sorting, half-open bounds, and monotone predicates.

Practice: lower bound/first occurrence; search rotated sorted input with a stated duplicate policy; integer square root with safe arithmetic; minimum feasible shipping capacity; merge intervals; meeting-room count. State whether touching intervals overlap.

**Gate:** Prove progress and termination, derive O(log n) versus O(n log n) correctly, and avoid index underflow and arithmetic overflow.

### C6 — Trees and heaps

Learn recursive and iterative traversal, call-stack costs, BST invariants, `BinaryHeap`, and `Reverse`.

Practice: tree depth; level order; validate BST with a duplicate policy; lowest common ancestor with missing-node semantics; tree serialization round trip; top-k values; merge k sorted sequences; median of a stream.

**Gate:** Handle empty and skewed trees, explain traversal state, and compare heap selection with sorting. Discuss recursion depth limits.

### C7 — Graphs

Learn adjacency lists, visited-state timing, BFS/DFS, topological sorting, union-find, and shortest-path assumptions.

Practice: islands; unweighted shortest path; clone an index-based graph; dependency ordering/cycle detection; connected components via union-find; Dijkstra on nonnegative weights. Study negative-edge failure and when Bellman–Ford is appropriate.

**Gate:** Define V/E complexity, cover disconnected graphs, cycles, self-loops, and repeated edges according to the contract, and select the algorithm from constraints.

### C8 — Backtracking and dynamic programming

Learn decision trees, pruning, state definition, recurrence, base cases, memoization, evaluation order, and reconstruction.

Practice: subsets; permutations with duplicates; combination search; grid word search; stairs; nonadjacent maximum sum; minimum coin count; grid paths with obstacles; longest common subsequence; word segmentation; 0/1 knapsack. Then consider interval DP and longest increasing subsequence variants.

**Gate:** Derive a recurrence instead of recalling code, distinguish unreachable states from zero, and explain when space compression destroys required information.

### C9 — Greedy, tries, bits, and advanced selection

Learn exchange arguments, prefix trees, bit manipulation, selection, and monotonic deques.

Practice: maximum nonoverlapping intervals; reachability jumps; trie prefix lookup; XOR single-value problem with explicit multiplicity assumptions; quickselect; sliding-window maximum. Add segment/Fenwick trees or advanced string matching only when role requirements or repeated gaps justify them.

**Gate:** Give a correctness argument or a counterexample to a tempting greedy rule. Explain signed/unsigned bit and overflow semantics in Rust.

### C10 — Mixed interview coding

Remove topic labels. Mix unfamiliar medium-complexity problems with selected harder combinations and practical tasks such as an event aggregator or bounded cache. Practice clarifying ambiguity, proposing a baseline, improving it, writing compilable code, and testing aloud.

**Gate:** Meet the coding readiness criteria in the [playbook](PLAYBOOK.md), including later recall and unfamiliar variants. Speed is measured only during explicitly chosen mock sessions; learning goals remain untimed.

## Systems and design sequence

Start S0 after C1. Design goals may be written artifacts while Rust fluency develops.

| Unit | Learn and practice | Evidence to advance |
| --- | --- | --- |
| S0: Request lifecycle | Trace DNS → connection/TLS → HTTP → service → database; distinguish latency, throughput, concurrency, and tail latency. Estimate QPS, payload bandwidth, and retained storage with units. | One request trace and a capacity estimate whose assumptions and arithmetic can be challenged. |
| S1: Data and operating systems | Processes/threads, memory, locks, deadlock, thread pools; SQL joins/indexes/query plans, transactions, isolation anomalies, relational versus key-value/document models. | Explain a lost-update scenario, choose a prevention mechanism, and justify an index for a stated query. |
| S2: Distributed foundations | Replication, partitioning, consistent hashing, consistency models, leader election/consensus concepts, quorums, clocks, caching, queues, retries, idempotency, backpressure. | Trace a retry after a timeout; prevent duplicate effects. Explain a partition tradeoff without treating CAP as an arbitrary “pick two.” |
| S3: Component and API design | Interfaces, data models, composition, ownership, errors, extensibility, test seams, concurrency safety. | Design then implement a bounded cache, rate limiter, or task scheduler in separate small goals. Test invariants and failure behavior. |
| S4: End-to-end design | Requirements, SLOs, estimates, APIs/schema, initial architecture, bottlenecks, and alternatives. | Design URL shortening, notification delivery, and a job queue separately; each includes one deep dive and a defended tradeoff. |
| S5: Scale and reliability | Fanout, ordering, hot keys, multi-region operation, failover, disaster recovery, observability, privacy, authorization, abuse controls, and cost. | Design a feed/chat service and an object-storage/file-sync service. Explain failure scenarios, recovery, and consistency at user-visible boundaries. |
| S6: Senior evolution | Legacy constraints, migration, schema compatibility, staged rollout, rollback, incident response, capacity planning, ownership, build/buy, and operational cost. | Produce a migration plan with verification and rollback, then defend it against changed requirements and a dependency outage. |

For every system, state the simplest viable design before adding scale mechanisms. Explain why a database, cache, queue, or partitioning strategy is needed. Include security, operations, and cost in the relevant decision rather than appending a generic checklist. See the playbook for a reusable design outline.

## Leadership and senior engineering sequence

Begin after C1 and reuse genuine work experience throughout; fictional scenarios may train judgment but must never be presented as past employment achievements.

| Unit | Assignment | Evidence to advance |
| --- | --- | --- |
| L0: Experience inventory | List three projects, your ownership, users, stakeholders, decisions, and known outcomes. Flag uncertain metrics. | Select one project and explain what you personally did. |
| L1: Story bank | Develop 8–10 reusable examples across ambiguity, disagreement, failure, mentoring, influence, delivery, customer impact, and a difficult technical decision. | Specific actions, alternatives, results, and lessons; sufficient coverage without inventing one unique story per prompt. |
| L2: Project deep dives | Prepare two project narratives: context → constraints → architecture → alternatives → execution → outcomes → lessons. | Defend design and impact under follow-ups, distinguish team outcomes from personal contributions, and identify what you would change. |
| L3: Senior judgment | Review code, handle an incident, negotiate scope, prioritize debt, give feedback, and write a short decision memo. | Make a clear recommendation, invite contrary evidence, explain risks and owners, and connect technical choices to customer/business outcomes. |
| L4: Delivery practice | Answer behavioral prompts aloud and handle interruptions, disagreement, and missing information. | Concise evidence-based answers with reflection, evaluated using the playbook. |

If your experience lacks examples of broader ownership, identify a real opportunity at work such as leading a migration, mentoring, or coordinating an incident improvement. Course exercises develop skill but cannot manufacture demonstrated senior scope.

## Specialization and company calibration

After S2 and L1, compare the target job descriptions with your evidence. Keep the shared coding/design/leadership foundation and choose one branch:

- **Backend/infrastructure (default):** distributed failure modes, storage, throughput, concurrency, observability, cost, and migrations.
- **Full-stack/frontend:** browser lifecycle, rendering and performance, accessibility, UI state, API contracts, frontend system design, and testing boundaries.
- **Mobile:** platform lifecycle, offline sync, persistence, networking, memory/battery constraints, and platform-specific language/API expectations.
- **ML/data:** data pipelines, training/serving boundaries, evaluation, leakage, experimentation, feature freshness, and model operations in addition to software fundamentals.

For Google, Meta, Amazon, Apple, and Netflix, create a brief only once a role is concrete: role/level, recruiter-confirmed rounds, permitted languages/tools, role-specific depth, assessment environment, relevant company values, and questions to ask. Do not assume equivalent titles or identical loops. In particular, confirm Rust support early; if the environment requires another language, schedule a translation goal using Java before mocks.

## Capstones and graduation

Use three capstones, broken into single-goal increments:

1. **Coding:** unfamiliar mixed problems with follow-ups, independent implementation, tests, and complexity reasoning.
2. **Design:** evolve a job-processing service from one machine to distributed workers. Address job state, duplicate execution, retries, cancellation, tenant fairness, monitoring, and migration. A design document and component prototype suffice; building an entire platform is not required.
3. **Leadership:** a real project deep dive plus an incident/tradeoff conversation with probing follow-ups.

Then run at least two complete mock loops on different problem sets, using the target recruiter's format if known. Track coding, architecture, leadership, and communication separately. Repair the weakest dimension with the smallest useful goal and reassess with a fresh prompt. Graduation means the [readiness evidence](PLAYBOOK.md#readiness-criteria) is met, not that every backlog item was checked off.
