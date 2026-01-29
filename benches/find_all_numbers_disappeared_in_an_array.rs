#[path = "../tests/find-all-numbers-disappeared-in-an-array.rs"]
mod find_all_numbers_disappeared_in_an_array;

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use find_all_numbers_disappeared_in_an_array::{
    find_disappeared_numbers_1, find_disappeared_numbers_2, find_disappeared_numbers_3,
    find_disappeared_numbers_4, find_disappeared_numbers_5, find_disappeared_numbers_6,
};

fn benchmark_find_disappeared(c: &mut Criterion) {
    let mut group = c.benchmark_group("find_disappeared_numbers");

    // Small input - leetcode example size
    let small: Vec<i32> = vec![4, 3, 2, 7, 8, 2, 3, 1];

    // Medium input - half missing (evens only)
    let medium: Vec<i32> = (1..=1000)
        .map(|x| if x % 2 == 0 { x } else { x + 1 })
        .collect();

    // Large input - sparse missing
    let large: Vec<i32> = (1..=10000)
        .map(|x| if x % 10 == 0 { 1 } else { x })
        .collect();

    // Small benchmarks
    group.bench_function("v1_u128/small", |b| {
        b.iter(|| find_disappeared_numbers_1(black_box(small.clone())))
    });
    group.bench_function("v2_sort/small", |b| {
        b.iter(|| find_disappeared_numbers_2(black_box(small.clone())))
    });
    group.bench_function("v3_hash/small", |b| {
        b.iter(|| find_disappeared_numbers_3(black_box(small.clone())))
    });
    group.bench_function("v4_inplace/small", |b| {
        b.iter(|| find_disappeared_numbers_4(black_box(small.clone())))
    });
    group.bench_function("v5_u64/small", |b| {
        b.iter(|| find_disappeared_numbers_5(black_box(small.clone())))
    });
    group.bench_function("v6_u64_filter/small", |b| {
        b.iter(|| find_disappeared_numbers_6(black_box(small.clone())))
    });

    // Medium benchmarks
    group.bench_function("v1_u128/medium", |b| {
        b.iter(|| find_disappeared_numbers_1(black_box(medium.clone())))
    });
    group.bench_function("v2_sort/medium", |b| {
        b.iter(|| find_disappeared_numbers_2(black_box(medium.clone())))
    });
    group.bench_function("v3_hash/medium", |b| {
        b.iter(|| find_disappeared_numbers_3(black_box(medium.clone())))
    });
    group.bench_function("v4_inplace/medium", |b| {
        b.iter(|| find_disappeared_numbers_4(black_box(medium.clone())))
    });
    group.bench_function("v5_u64/medium", |b| {
        b.iter(|| find_disappeared_numbers_5(black_box(medium.clone())))
    });
    group.bench_function("v6_u64_filter/medium", |b| {
        b.iter(|| find_disappeared_numbers_6(black_box(medium.clone())))
    });

    // Large benchmarks
    group.bench_function("v1_u128/large", |b| {
        b.iter(|| find_disappeared_numbers_1(black_box(large.clone())))
    });
    group.bench_function("v2_sort/large", |b| {
        b.iter(|| find_disappeared_numbers_2(black_box(large.clone())))
    });
    group.bench_function("v3_hash/large", |b| {
        b.iter(|| find_disappeared_numbers_3(black_box(large.clone())))
    });
    group.bench_function("v4_inplace/large", |b| {
        b.iter(|| find_disappeared_numbers_4(black_box(large.clone())))
    });
    group.bench_function("v5_u64/large", |b| {
        b.iter(|| find_disappeared_numbers_5(black_box(large.clone())))
    });
    group.bench_function("v6_u64_filter/large", |b| {
        b.iter(|| find_disappeared_numbers_6(black_box(large.clone())))
    });

    group.finish();
}

criterion_group!(benches, benchmark_find_disappeared);
criterion_main!(benches);
