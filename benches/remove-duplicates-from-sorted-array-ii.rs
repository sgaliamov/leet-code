#[path = "../tests/remove-duplicates-from-sorted-array-ii.rs"]
mod remove_duplicates_from_sorted_array_ii;

use criterion::{criterion_group, criterion_main, Criterion};
use remove_duplicates_from_sorted_array_ii::{
    remove_duplicates_1, remove_duplicates_2, remove_duplicates_3,
};
use std::hint::black_box;

fn benchmark_remove_duplicates(c: &mut Criterion) {
    let small_input = vec![1, 1, 1, 2, 2, 3];
    let medium_input = vec![0, 0, 1, 1, 1, 1, 2, 3, 3, 4, 4, 4, 5, 5, 6, 7, 7, 7, 7];
    let large_input = {
        let mut v = Vec::with_capacity(1000);
        for i in 0..250 {
            v.push(i);
            v.push(i);
            v.push(i);
            v.push(i);
        }
        v
    };

    let worst_case = vec![1; 1000]; // All same value

    let mut group = c.benchmark_group("remove_duplicates");

    // v1 - tracking count and value
    group.bench_function("v1_count/small", |b| {
        b.iter(|| {
            let mut input = black_box(small_input.clone());
            remove_duplicates_1(&mut input)
        })
    });

    group.bench_function("v1_count/medium", |b| {
        b.iter(|| {
            let mut input = black_box(medium_input.clone());
            remove_duplicates_1(&mut input)
        })
    });

    group.bench_function("v1_count/large", |b| {
        b.iter(|| {
            let mut input = black_box(large_input.clone());
            remove_duplicates_1(&mut input)
        })
    });

    group.bench_function("v1_count/worst_case", |b| {
        b.iter(|| {
            let mut input = black_box(worst_case.clone());
            remove_duplicates_1(&mut input)
        })
    });

    // v2 - nested if/else with p-2 comparison
    group.bench_function("v2_nested/small", |b| {
        b.iter(|| {
            let mut input = black_box(small_input.clone());
            remove_duplicates_2(&mut input)
        })
    });

    group.bench_function("v2_nested/medium", |b| {
        b.iter(|| {
            let mut input = black_box(medium_input.clone());
            remove_duplicates_2(&mut input)
        })
    });

    group.bench_function("v2_nested/large", |b| {
        b.iter(|| {
            let mut input = black_box(large_input.clone());
            remove_duplicates_2(&mut input)
        })
    });

    group.bench_function("v2_nested/worst_case", |b| {
        b.iter(|| {
            let mut input = black_box(worst_case.clone());
            remove_duplicates_2(&mut input)
        })
    });

    // v3 - simplified with conditional write
    group.bench_function("v3_simple/small", |b| {
        b.iter(|| {
            let mut input = black_box(small_input.clone());
            remove_duplicates_3(&mut input)
        })
    });

    group.bench_function("v3_simple/medium", |b| {
        b.iter(|| {
            let mut input = black_box(medium_input.clone());
            remove_duplicates_3(&mut input)
        })
    });

    group.bench_function("v3_simple/large", |b| {
        b.iter(|| {
            let mut input = black_box(large_input.clone());
            remove_duplicates_3(&mut input)
        })
    });

    group.bench_function("v3_simple/worst_case", |b| {
        b.iter(|| {
            let mut input = black_box(worst_case.clone());
            remove_duplicates_3(&mut input)
        })
    });

    group.finish();
}

criterion_group!(benches, benchmark_remove_duplicates);
criterion_main!(benches);
