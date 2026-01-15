use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

fn current_version(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::with_capacity(nums.len() * 2);
    ans.extend_from_slice(&nums);
    ans.extend_from_slice(&nums);
    ans
}

fn unsafe_version(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut ans = Vec::with_capacity(len * 2);
    unsafe {
        let src = nums.as_ptr();
        let dst = ans.as_mut_ptr();
        std::ptr::copy_nonoverlapping(src, dst, len);
        std::ptr::copy_nonoverlapping(src, dst.add(len), len);
        ans.set_len(len * 2);
    }
    ans
}

fn benchmark_concatenation(c: &mut Criterion) {
    let small_input = vec![1, 2, 3, 4, 5];
    let medium_input = vec![1; 100];
    let large_input = vec![1; 1000];

    let mut group = c.benchmark_group("concatenation");

    group.bench_function("current/small", |b| {
        b.iter(|| current_version(black_box(small_input.clone())))
    });

    group.bench_function("unsafe/small", |b| {
        b.iter(|| unsafe_version(black_box(small_input.clone())))
    });

    group.bench_function("current/medium", |b| {
        b.iter(|| current_version(black_box(medium_input.clone())))
    });

    group.bench_function("unsafe/medium", |b| {
        b.iter(|| unsafe_version(black_box(medium_input.clone())))
    });

    group.bench_function("current/large", |b| {
        b.iter(|| current_version(black_box(large_input.clone())))
    });

    group.bench_function("unsafe/large", |b| {
        b.iter(|| unsafe_version(black_box(large_input.clone())))
    });

    group.finish();
}

criterion_group!(benches, benchmark_concatenation);
criterion_main!(benches);
