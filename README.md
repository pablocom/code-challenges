# code-challenges (Rust edition)

A collection of **114 algorithm and data-structure solutions** in idiomatic
Rust, organized by category, each with unit tests living right beside the
implementation. This is a Rust port of a long-running C#/.NET practice
repository — same problems, re-expressed the Rust way.

## Categories

| Category | Solutions |
|---|---|
| Arrays | 31 |
| Strings | 20 |
| DynamicProgramming | 12 |
| Graphs | 9 |
| LinkedLists | 8 |
| BinaryTrees | 8 |
| Backtracking | 8 |
| Mathematics | 7 |
| DataStructures | 5 |
| Searching | 5 |
| Concurrency | 1 |

260 tests, all passing.

## Prerequisites

- A stable Rust toolchain (edition 2024 — Rust 1.85 or newer).

## Getting started

```bash
# Build
cargo build

# Run all tests
cargo test

# Run the tests for one category or one problem
cargo test arrays::
cargo test two_sum

# Lint and format
cargo clippy --all-targets
cargo fmt

# Run the benchmarks (Criterion) for selected algorithms
cargo bench
```

## How it's organized

```
src/
  lib.rs                 # declares the category modules
  arrays/
    mod.rs               # declares the problem modules (and any shared types)
    two_sum.rs           # one problem per file: implementation + inline tests
    ...
  linked_lists/
    mod.rs               # shared ListNode + builders
    ...
  ...
```

Conventions:

- **One file per problem.** Each holds the solution and a
  `#[cfg(test)] mod tests` block — the idiomatic Rust home for unit tests, so
  the tests and the code they exercise never drift apart.
- **Shared helper types** (linked-list / tree / graph nodes and their builders)
  live at the top of their category module.
- Entry points are usually named `solve`, or something more descriptive when a
  problem has several approaches (e.g. `solve_with_min_heap`,
  `solve_optimized`).

## Notes on the port

A few problems exercise Rust's ownership model in interesting ways:

- **Linked lists** use `Option<Box<ListNode>>`; the random-pointer deep-copy
  uses `Rc<RefCell<_>>` with `Weak` back-references to avoid cycles.
- **Binary trees** use `Option<Box<TreeNode>>` with a fluent builder; the AVL
  tree is generic over `T: Ord + Clone`.
- **The LRU cache** is an index-based arena (a `Vec<Node>` with a free list)
  rather than `Rc<RefCell>`, keeping `get`/`put` at O(1) in safe Rust.
- **Print in Order** replaces C# semaphores with `Mutex` + `Condvar` latches.
- **Max-heap / Sudoku** and other in-place algorithms keep their original
  shape, while a handful of solutions were simplified where idiomatic Rust
  (iterators, pattern matching, `?`) reads more clearly than a literal
  translation.
