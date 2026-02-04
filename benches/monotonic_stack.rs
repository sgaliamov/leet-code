use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use leet_code::monotonic_stack::DecreasingStack;
use std::hint::black_box;

/// Safe reference implementation for comparison
struct DecreasingStackSafe<T> {
    stack: Vec<T>,
}

impl<T: PartialOrd> DecreasingStackSafe<T> {
    #[inline]
    fn with_capacity(capacity: usize) -> Self {
        Self {
            stack: Vec::with_capacity(capacity),
        }
    }

    #[inline]
    fn push(&mut self, val: T) {
        while let Some(last) = self.stack.last() {
            if last > &val {
                break;
            }

            if last == &val {
                return;
            }
        }
        self.stack.push(val);
    }
}

fn benchmark_safe(c: &mut Criterion) {
    let mut group = c.benchmark_group("monotonic_stack_safe");

    for size in [100, 1000, 10000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter(|| {
                let mut stack = DecreasingStackSafe::with_capacity(size);
                for i in 0..size {
                    stack.push(black_box(i % 100));
                }
                black_box(stack)
            });
        });
    }
    group.finish();
}

fn benchmark_unsafe(c: &mut Criterion) {
    let mut group = c.benchmark_group("monotonic_stack_unsafe");

    for size in [100, 1000, 10000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter(|| {
                let mut stack = DecreasingStack::with_capacity(size);
                for i in 0..size {
                    stack.push(black_box(i % 100));
                }
                black_box(stack)
            });
        });
    }
    group.finish();
}

fn benchmark_worst_case_safe(c: &mut Criterion) {
    let mut group = c.benchmark_group("worst_case_safe");

    for size in [100, 1000, 10000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter(|| {
                let mut stack = DecreasingStackSafe::with_capacity(size);
                // Worst case: push increasing values (each push clears entire stack)
                for i in 0..size {
                    stack.push(black_box(i));
                }
                black_box(stack)
            });
        });
    }
    group.finish();
}

fn benchmark_worst_case_unsafe(c: &mut Criterion) {
    let mut group = c.benchmark_group("worst_case_unsafe");

    for size in [100, 1000, 10000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter(|| {
                let mut stack = DecreasingStack::with_capacity(size);
                // Worst case: push increasing values (each push clears entire stack)
                for i in 0..size {
                    stack.push(black_box(i));
                }
                black_box(stack)
            });
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    benchmark_safe,
    benchmark_unsafe,
    benchmark_worst_case_safe,
    benchmark_worst_case_unsafe
);
criterion_main!(benches);
