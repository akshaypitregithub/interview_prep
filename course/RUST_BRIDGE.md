# Rust Bridge for a Java Developer

## For G001

Read these concepts, then return to the exercise. You do not need advanced lifetimes, traits, or smart pointers yet.

| Rust | Meaning | Java comparison / caution |
| --- | --- | --- |
| `fn name(value: i32) -> bool` | A function taking a signed 32-bit integer and returning a Boolean. | Parameter types follow names; the return type follows `->`. |
| `let value = 3;` / `let mut value = 3;` | Bindings are immutable unless marked `mut`. | Mutability is explicit rather than the default for local variables. |
| `&[i32]` | A borrowed, read-only view of contiguous integers. | Think of array access without taking ownership; Rust enforces the borrowing rules. |
| `usize` | The unsigned type used for indices and lengths. | Avoid using `i32` for indexing or blindly converting negative numbers. |
| `Option<usize>` | Either `Some(index)` or `None`. | Represents absence without `null` or a sentinel such as `-1`. |
| `return value;` | Exit a function with a value. | Similar to Java; Rust can also return the final expression without a semicolon. |
| `assert_eq!(actual, expected)` | A test assertion; `!` indicates a macro. | Similar purpose to an equality assertion in JUnit. |
| `todo!("message")` | A compiling placeholder that panics when called. | Your exercise is incomplete until this is replaced. |

An unrelated syntax example:

```rust
fn sign_label(value: i32) -> &'static str {
    if value < 0 {
        "negative"
    } else {
        "nonnegative"
    }
}
```

String literals in this example have a `'static` lifetime; understanding lifetime annotations is not required for G001. A `for` loop uses `for item in collection { ... }`; a `while` loop uses `while condition { ... }`. A shared iterator may yield references. Ask for a focused syntax example if the compiler reports a reference/value mismatch.

Read a compiler error from the first diagnostic downward: location, expected type, actual type, then suggested fix. Do not apply `.clone()` or casts automatically just to silence it.

## Learn later, when needed

| Topic | Rust focus | Common Java habit to revisit |
| --- | --- | --- |
| Ownership | Moves, borrowing, `Copy` versus `Clone`, scope and destruction. | Assigning a collection can transfer ownership rather than create another usable reference. |
| Collections | `Vec`, `HashMap`, `HashSet`, `VecDeque`, `BinaryHeap`, and `entry`. | Heap direction, iteration order, and map update APIs differ. |
| Text | `String` / `&str`, `.bytes()`, `.chars()`, explicit ASCII contracts. | Neither Java UTF-16 indexing nor Rust byte indexing inherently identifies a displayed character. |
| Errors | `Option`, `Result`, pattern matching, `?`. | Recoverable errors are usually values, not exceptions; avoid unproven `unwrap()`. |
| Data models | Structs, enums, traits, generics, composition. | Do not translate every Java class hierarchy into inheritance. |
| Trees/lists | `Box`, `Option::take`, or index-based arenas; shared ownership only when required. | Borrowing and ownership constrain pointer-heavy designs. |
| Concurrency | Threads, channels, `Arc`, `Mutex`, `Send`/`Sync`; async later if relevant. | Memory safety does not eliminate deadlocks or logical races. |

Use the [Rust Book](https://doc.rust-lang.org/book/) for chapters on common concepts, ownership, enums, collections, errors, and tests. Use the [standard library reference](https://doc.rust-lang.org/std/) to check exact APIs. Read only what the active goal needs; syntax lookup is allowed during learning and logged separately from algorithm hints.
