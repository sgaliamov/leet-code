#[path = "../tests/set-mismatch.rs"]
mod set_mismatch;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use set_mismatch::*;
use std::hint::black_box;

fn generate_test_data(size: usize) -> Vec<i32> {
    let mut nums: Vec<i32> = (1..=size as i32).collect();
    // Duplicate the middle element and remove the last one
    let middle = size / 2;
    nums[middle] = nums[middle - 1];
    nums.truncate(size);
    nums
}

// Benchmark results (sorted by performance for each size):
//
// n=100:
//   1. sum_of_squares:  ~58ns   (FASTEST - pure math, minimal operations)
//   2. sum_formula:     ~137ns
//   3. bool_array:      ~148ns
//   4. xor_trick:       ~267ns
//   5. hashset:         ~2.7µs  (SLOWEST - hashing overhead)
//
// n=1,000:
//   1. sum_of_squares:  ~135ns  (FASTEST - constant time regardless of input size!)
//   2. sum_formula:     ~651ns
//   3. bool_array:      ~744ns
//   4. xor_trick:       ~2.1µs
//   5. hashset:         ~31µs   (SLOWEST)
//
// n=10,000:
//   1. sum_of_squares:  ~1.3µs  (FASTEST - scales linearly with input)
//   2. sum_formula:     ~6.2µs
//   3. bool_array:      ~7.4µs
//   4. xor_trick:       ~21µs
//   5. hashset:         ~271µs  (SLOWEST)
//
// Winner: sum_of_squares is the clear champion across all input sizes.
// The mathematical approach with sum of squares formula (solution 5) dominates
// due to minimal operations and excellent cache behavior.
fn benchmark_set_mismatch(c: &mut Criterion) {
    let mut group = c.benchmark_group("set_mismatch");

    for size in [100, 1000, 10000] {
        let nums = generate_test_data(size);

        group.bench_with_input(
            BenchmarkId::new("hashset", size),
            &nums,
            |b, nums| {
                b.iter(|| find_error_nums_1(black_box(nums.clone())));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("bool_array", size),
            &nums,
            |b, nums| {
                b.iter(|| find_error_nums_2(black_box(nums.clone())));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("sum_formula", size),
            &nums,
            |b, nums| {
                b.iter(|| find_error_nums_3(black_box(nums.clone())));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("xor_trick", size),
            &nums,
            |b, nums| {
                b.iter(|| find_error_nums_4(black_box(nums.clone())));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("sum_of_squares", size),
            &nums,
            |b, nums| {
                b.iter(|| find_error_nums_5(black_box(nums.clone())));
            },
        );
    }

    group.finish();
}

criterion_group!(benches, benchmark_set_mismatch);
criterion_main!(benches);
