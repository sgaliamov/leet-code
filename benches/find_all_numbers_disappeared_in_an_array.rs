#[path = "../tests/find-all-numbers-disappeared-in-an-array.rs"]
mod find_all_numbers_disappeared_in_an_array;

use criterion::{criterion_group, criterion_main, Criterion, black_box};
use find_all_numbers_disappeared_in_an_array::{
    find_disappeared_numbers_1,
    find_disappeared_numbers_2,
    find_disappeared_numbers_3,
    find_disappeared_numbers_4,
    find_disappeared_numbers_5,
};

fn benchmark_find_disappeared(c: &mut Criterion) {
    let mut group = c.benchmark_group("find_disappeared_numbers");

    // Dataset generation
    // 1. Full array (no missing)
    // 2. Many missing
    // 3. Random missing

    let n = 10000;

    // Case 1: No missing numbers (Best case for some, worst for others?)
    let input_full: Vec<i32> = (1..=n as i32).collect();

    // Case 2: Half missing (evens only)
    let input_half: Vec<i32> = (1..=n as i32).map(|x| if x % 2 == 0 { x } else { x + 1 }).collect();
    // This creates duplicates and misses odds. Wait, map keeps length n.
    // 1 -> 2, 2 -> 2, 3 -> 4, 4 -> 4...
    // Input is [2, 2, 4, 4, ...]. Missing [1, 3, 5...]

    group.bench_function("v1_u128_bitset", |b| {
        b.iter(|| find_disappeared_numbers_1(black_box(input_half.clone())))
    });

    group.bench_function("v2_sort_dedup", |b| {
        b.iter(|| find_disappeared_numbers_2(black_box(input_half.clone())))
    });

    group.bench_function("v3_hashset", |b| {
        b.iter(|| find_disappeared_numbers_3(black_box(input_half.clone())))
    });

    group.bench_function("v4_inplace_negation", |b| {
        b.iter(|| find_disappeared_numbers_4(black_box(input_half.clone())))
    });

    group.bench_function("v5_optimized_bitset", |b| {
        b.iter(|| find_disappeared_numbers_5(black_box(input_half.clone())))
    });

    group.finish();
}

criterion_group!(benches, benchmark_find_disappeared);
criterion_main!(benches);
