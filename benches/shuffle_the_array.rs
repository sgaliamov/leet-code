#[path = "../tests/shuffle-the-array.rs"]
mod shuffle_the_array;

use criterion::{Criterion, criterion_group, criterion_main};
use shuffle_the_array::shuffle_1 as current_version;
use std::hint::black_box;

fn with_capacity_push(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let n = n as usize;
    let mut ans = Vec::with_capacity(n * 2);

    for i in 0..n {
        ans.push(nums[i]);
        ans.push(nums[i + n]);
    }

    ans
}

fn unsafe_version(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let n = n as usize;
    let mut ans: Vec<i32> = Vec::with_capacity(n * 2);

    unsafe {
        let dst = ans.as_mut_ptr();
        for i in 0..n {
            *dst.add(i * 2) = nums[i];
            *dst.add(i * 2 + 1) = nums[i + n];
        }
        ans.set_len(n * 2);
    }

    ans
}

fn iterator_version(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let n = n as usize;
    nums[..n]
        .iter()
        .zip(&nums[n..])
        .flat_map(|(&x, &y)| [x, y])
        .collect()
}

fn benchmark_shuffle(c: &mut Criterion) {
    let small_input = (vec![1, 2, 3, 4, 5, 6], 3);
    let medium_input = (vec![1; 200], 100);
    let large_input = (vec![1; 2000], 1000);

    let mut group = c.benchmark_group("shuffle");

    group.bench_function("current_small", |b| {
        b.iter(|| current_version(black_box(small_input.0.clone()), black_box(small_input.1)))
    });

    group.bench_function("push_small", |b| {
        b.iter(|| with_capacity_push(black_box(small_input.0.clone()), black_box(small_input.1)))
    });

    group.bench_function("unsafe_small", |b| {
        b.iter(|| unsafe_version(black_box(small_input.0.clone()), black_box(small_input.1)))
    });

    group.bench_function("iterator_small", |b| {
        b.iter(|| iterator_version(black_box(small_input.0.clone()), black_box(small_input.1)))
    });

    group.bench_function("current_medium", |b| {
        b.iter(|| current_version(black_box(medium_input.0.clone()), black_box(medium_input.1)))
    });

    group.bench_function("push_medium", |b| {
        b.iter(|| with_capacity_push(black_box(medium_input.0.clone()), black_box(medium_input.1)))
    });

    group.bench_function("unsafe_medium", |b| {
        b.iter(|| unsafe_version(black_box(medium_input.0.clone()), black_box(medium_input.1)))
    });

    group.bench_function("iterator_medium", |b| {
        b.iter(|| iterator_version(black_box(medium_input.0.clone()), black_box(medium_input.1)))
    });

    group.bench_function("current_large", |b| {
        b.iter(|| current_version(black_box(large_input.0.clone()), black_box(large_input.1)))
    });

    group.bench_function("push_large", |b| {
        b.iter(|| with_capacity_push(black_box(large_input.0.clone()), black_box(large_input.1)))
    });

    group.bench_function("unsafe_large", |b| {
        b.iter(|| unsafe_version(black_box(large_input.0.clone()), black_box(large_input.1)))
    });

    group.bench_function("iterator_large", |b| {
        b.iter(|| iterator_version(black_box(large_input.0.clone()), black_box(large_input.1)))
    });

    group.finish();
}

criterion_group!(benches, benchmark_shuffle);
criterion_main!(benches);
