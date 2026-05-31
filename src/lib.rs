//! # Code Challenges (Rust edition)
//!
//! A collection of algorithm and data-structure solutions, organized by
//! category, ported from a long-running C#/.NET practice repository to
//! idiomatic Rust.
//!
//! ## Conventions
//!
//! - Each category is a module (`arrays`, `strings`, `graphs`, ...).
//! - Each problem lives in its own file with a `solve` (or aptly named)
//!   entry point.
//! - Tests live *next to* the implementation in a `#[cfg(test)] mod tests`
//!   block — the idiomatic Rust home for unit tests.
//! - Shared helper types (linked-list / tree / graph nodes and their
//!   builders) live at the top of their category module.

pub mod arrays;
pub mod backtracking;
pub mod binary_trees;
pub mod concurrency;
pub mod data_structures;
pub mod dynamic_programming;
pub mod graphs;
pub mod linked_lists;
pub mod mathematics;
pub mod searching;
pub mod strings;
