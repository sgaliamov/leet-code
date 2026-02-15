use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::hint::black_box;

#[path = "../tests/ransom-note.rs"]
mod ransom_note;

use ransom_note::*;

fn benchmark_ransom_note(c: &mut Criterion) {
    let mut group = c.benchmark_group("ransom_note");

    // Small case: short strings, early success
    let small_match = ("abc".to_string(), "dcba".to_string());
    group.bench_with_input(
        BenchmarkId::new("can_construct_4_small_match", "abc/dcba"),
        &small_match,
        |b, (r, m)| b.iter(|| can_construct_4(black_box(r.clone()), black_box(m.clone()))),
    );
    group.bench_with_input(
        BenchmarkId::new("can_construct_5_small_match", "abc/dcba"),
        &small_match,
        |b, (r, m)| b.iter(|| can_construct_5(black_box(r.clone()), black_box(m.clone()))),
    );
    group.bench_with_input(
        BenchmarkId::new("can_construct_6_small_match", "abc/dcba"),
        &small_match,
        |b, (r, m)| b.iter(|| can_construct_6(black_box(r.clone()), black_box(m.clone()))),
    );
    group.bench_with_input(
        BenchmarkId::new("can_construct_7_small_match", "abc/dcba"),
        &small_match,
        |b, (r, m)| b.iter(|| can_construct_7(black_box(r.clone()), black_box(m.clone()))),
    );

    // Medium case: typical use
    let medium_match = (
        "thequickbrownfox".to_string(),
        "thequickbrownfoxjumpsoverthelazydog".to_string(),
    );
    group.bench_with_input(
        BenchmarkId::new("can_construct_4_medium_match", "medium"),
        &medium_match,
        |b, (r, m)| b.iter(|| can_construct_4(black_box(r.clone()), black_box(m.clone()))),
    );
    group.bench_with_input(
        BenchmarkId::new("can_construct_5_medium_match", "medium"),
        &medium_match,
        |b, (r, m)| b.iter(|| can_construct_5(black_box(r.clone()), black_box(m.clone()))),
    );
    group.bench_with_input(
        BenchmarkId::new("can_construct_6_medium_match", "medium"),
        &medium_match,
        |b, (r, m)| b.iter(|| can_construct_6(black_box(r.clone()), black_box(m.clone()))),
    );
    group.bench_with_input(
        BenchmarkId::new("can_construct_7_medium_match", "medium"),
        &medium_match,
        |b, (r, m)| b.iter(|| can_construct_7(black_box(r.clone()), black_box(m.clone()))),
    );

    // Worst case for lazy approach: ransom note in reverse alphabet order
    let worst_case = (
        "zyxwvutsrqponmlkjihgfedcba".to_string(),
        "abcdefghijklmnopqrstuvwxyz".to_string(),
    );
    group.bench_with_input(
        BenchmarkId::new("can_construct_4_worst", "reverse_alpha"),
        &worst_case,
        |b, (r, m)| b.iter(|| can_construct_4(black_box(r.clone()), black_box(m.clone()))),
    );
    group.bench_with_input(
        BenchmarkId::new("can_construct_5_worst", "reverse_alpha"),
        &worst_case,
        |b, (r, m)| b.iter(|| can_construct_5(black_box(r.clone()), black_box(m.clone()))),
    );
    group.bench_with_input(
        BenchmarkId::new("can_construct_6_worst", "reverse_alpha"),
        &worst_case,
        |b, (r, m)| b.iter(|| can_construct_6(black_box(r.clone()), black_box(m.clone()))),
    );
    group.bench_with_input(
        BenchmarkId::new("can_construct_7_worst", "reverse_alpha"),
        &worst_case,
        |b, (r, m)| b.iter(|| can_construct_7(black_box(r.clone()), black_box(m.clone()))),
    );

    // Early failure case
    let early_fail = ("xyz".to_string(), "abc".to_string());
    group.bench_with_input(
        BenchmarkId::new("can_construct_4_early_fail", "xyz/abc"),
        &early_fail,
        |b, (r, m)| b.iter(|| can_construct_4(black_box(r.clone()), black_box(m.clone()))),
    );
    group.bench_with_input(
        BenchmarkId::new("can_construct_5_early_fail", "xyz/abc"),
        &early_fail,
        |b, (r, m)| b.iter(|| can_construct_5(black_box(r.clone()), black_box(m.clone()))),
    );
    group.bench_with_input(
        BenchmarkId::new("can_construct_6_early_fail", "xyz/abc"),
        &early_fail,
        |b, (r, m)| b.iter(|| can_construct_6(black_box(r.clone()), black_box(m.clone()))),
    );
    group.bench_with_input(
        BenchmarkId::new("can_construct_7_early_fail", "xyz/abc"),
        &early_fail,
        |b, (r, m)| b.iter(|| can_construct_7(black_box(r.clone()), black_box(m.clone()))),
    );

    // Best case for bitflag: short ransom note, very long magazine
    let long_magazine = (
        "abc".to_string(),
        "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz"
            .to_string(),
    );
    group.bench_with_input(
        BenchmarkId::new("can_construct_5_long_mag", "abc/long"),
        &long_magazine,
        |b, (r, m)| b.iter(|| can_construct_5(black_box(r.clone()), black_box(m.clone()))),
    );
    group.bench_with_input(
        BenchmarkId::new("can_construct_6_long_mag", "abc/long"),
        &long_magazine,
        |b, (r, m)| b.iter(|| can_construct_6(black_box(r.clone()), black_box(m.clone()))),
    );
    group.bench_with_input(
        BenchmarkId::new("can_construct_7_long_mag", "abc/long"),
        &long_magazine,
        |b, (r, m)| b.iter(|| can_construct_7(black_box(r.clone()), black_box(m.clone()))),
    );

    group.finish();
}

criterion_group!(benches, benchmark_ransom_note);
criterion_main!(benches);
