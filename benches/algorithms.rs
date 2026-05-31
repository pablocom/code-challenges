//! Criterion benchmarks for a handful of representative algorithms — the Rust
//! counterpart of the original repository's BenchmarkDotNet project.
//!
//! Run with `cargo bench`.

use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

use code_challenges::arrays::kth_largest;
use code_challenges::dynamic_programming::coin_change;
use code_challenges::graphs::number_of_islands;
use code_challenges::strings::longest_substring_palindrome;

fn bench_kth_largest(c: &mut Criterion) {
    let nums: Vec<i32> = (0..10_000).map(|i| (i * 7919) % 10_000).collect();
    c.bench_function("kth_largest::sort", |b| {
        b.iter(|| kth_largest::solve_with_sort(black_box(&nums), black_box(500)))
    });
    c.bench_function("kth_largest::heap", |b| {
        b.iter(|| kth_largest::solve_with_max_heap(black_box(&nums), black_box(500)))
    });
}

fn bench_coin_change(c: &mut Criterion) {
    let coins = [186, 419, 83, 408];
    c.bench_function("coin_change", |b| {
        b.iter(|| coin_change::solve(black_box(&coins), black_box(6249)))
    });
}

fn bench_number_of_islands(c: &mut Criterion) {
    let grid: Vec<String> = (0..200)
        .map(|r| {
            (0..200)
                .map(|col| if (r + col) % 2 == 0 { '1' } else { '0' })
                .collect()
        })
        .collect();
    let rows: Vec<&str> = grid.iter().map(String::as_str).collect();
    c.bench_function("number_of_islands", |b| {
        b.iter(|| number_of_islands::solve(black_box(&rows)))
    });
}

fn bench_longest_palindrome(c: &mut Criterion) {
    let text = "forgeeksskeegfor".repeat(40);
    c.bench_function("longest_substring_palindrome", |b| {
        b.iter(|| longest_substring_palindrome::solve(black_box(&text)))
    });
}

criterion_group!(
    benches,
    bench_kth_largest,
    bench_coin_change,
    bench_number_of_islands,
    bench_longest_palindrome
);
criterion_main!(benches);
