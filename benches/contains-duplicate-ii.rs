use criterion::{Criterion, criterion_group, criterion_main};
use std::collections::HashSet;
use std::hash::{BuildHasherDefault, DefaultHasher};
use std::hint::black_box;

fn with_random_state(nums: Vec<i32>, k: i32) -> bool {
    let k = k as usize;
    let mut set = HashSet::new();

    for i in 0..nums.len() {
        let n = nums[i];

        if !set.insert(n) {
            return true;
        }

        if k <= i {
            set.remove(&nums[i - k]);
        }
    }

    false
}

fn with_default_hasher(nums: Vec<i32>, k: i32) -> bool {
    let k = k as usize;
    let mut set = HashSet::with_hasher(BuildHasherDefault::<DefaultHasher>::default());

    for i in 0..nums.len() {
        let n = nums[i];

        if !set.insert(n) {
            return true;
        }

        if k <= i {
            set.remove(&nums[i - k]);
        }
    }

    false
}

fn benchmark_hashers(c: &mut Criterion) {
    let mut group = c.benchmark_group("hasher_comparison");

    // Small window, no duplicates found
    let small_input = (1..=100).collect::<Vec<_>>();
    let k = 10;

    group.bench_function("random_state_small", |b| {
        b.iter(|| with_random_state(black_box(small_input.clone()), black_box(k)))
    });

    group.bench_function("default_hasher_small", |b| {
        b.iter(|| with_default_hasher(black_box(small_input.clone()), black_box(k)))
    });

    // Large window, no duplicates found
    let large_input = (1..=10000).collect::<Vec<_>>();
    let k = 1000;

    group.bench_function("random_state_large", |b| {
        b.iter(|| with_random_state(black_box(large_input.clone()), black_box(k)))
    });

    group.bench_function("default_hasher_large", |b| {
        b.iter(|| with_default_hasher(black_box(large_input.clone()), black_box(k)))
    });

    group.finish();
}

criterion_group!(benches, benchmark_hashers);
criterion_main!(benches);
