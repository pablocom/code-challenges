# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

A Rust (edition 2024) library crate of algorithm and data-structure solutions,
organized by category (Arrays, Strings, Graphs, DynamicProgramming, ...). It is
a port of a C#/.NET practice repository. The crate has no runtime dependencies.

## Build & test commands

```bash
cargo build                              # Build the crate
cargo test                               # Run all tests
cargo test <category>::                  # e.g. cargo test graphs::
cargo test <problem_name>                # e.g. cargo test two_sum
cargo clippy --all-targets               # Lint
cargo fmt                                # Format
```

## Architecture

- `src/lib.rs` declares the category modules.
- Each category is a directory module (`src/arrays/`, `src/graphs/`, ...) whose
  `mod.rs` declares one submodule per problem and holds any shared types
  (e.g. `ListNode`, `TreeNode`, builders).
- Each problem is one file with the solution and a colocated
  `#[cfg(test)] mod tests` block.

## Conventions

- Add a new problem as `src/{category}/{problem_name}.rs`, declare it in that
  category's `mod.rs`, and put its tests in the same file.
- Public entry points are named `solve` by default; use descriptive names when
  a problem offers multiple approaches (`solve_with_min_heap`, `tabulation`,
  `memoization`, ...).
- Keep it clippy-clean and `cargo fmt`-formatted. Prefer self-explanatory names
  and structure over comments; reserve comments for *why*, not *what*.
- Tests should cover the same scenarios as the problem demands, including edge
  cases (empty input, single element, overflow boundaries).
