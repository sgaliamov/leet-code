#[path = "../tests/valid-palindrome.rs"]
mod valid_palindrome;

use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use valid_palindrome::{is_palindrome_1, is_palindrome_2, is_palindrome_3, is_palindrome_4};

fn is_palindrome_iterators(s: String) -> bool {
    let bytes = s.as_bytes();
    let mut left = bytes
        .iter()
        .enumerate()
        .filter(|(_, b)| b.is_ascii_alphanumeric());
    let mut right = bytes
        .iter()
        .enumerate()
        .rev()
        .filter(|(_, b)| b.is_ascii_alphanumeric());

    loop {
        match (left.next(), right.next()) {
            (Some((i, a)), Some((j, b))) => {
                if i >= j {
                    return true;
                }
                if !a.eq_ignore_ascii_case(b) {
                    return false;
                }
            }
            _ => return true,
        }
    }
}

fn is_palindrome_bytes_vec(s: String) -> bool {
    let bytes: Vec<u8> = s
        .bytes()
        .filter(|b| b.is_ascii_alphanumeric())
        .map(|b| b.to_ascii_lowercase())
        .collect();

    if bytes.is_empty() {
        return true;
    }

    let mut i = 0;
    let mut j = bytes.len() - 1;

    while i < j {
        if bytes[i] != bytes[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }

    true
}

fn benchmark_palindrome(c: &mut Criterion) {
    let mut group = c.benchmark_group("valid_palindrome");

    let test_cases = vec![
        ("short_true", "A man, a plan, a canal: Panama"),
        ("short_false", "race a car"),
        (
            "medium_true",
            "Was it a car or a cat I saw? No way! A Toyota's a Toyota!",
        ),
        (
            "medium_false",
            "This is definitely not a palindrome at all, no sir!",
        ),
        (
            "long_true",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa \
             aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa \
             aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa \
             aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa \
             aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa \
             aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa \
             aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa \
             aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa \
             aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa \
             aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa \
             aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa \
             aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        ),
        (
            "long_false",
            "A man, a plan, a caretaker, a moksha, Lufkin, a jacinth, Gile, Daniell, Ivanov, \
             an odor, a negativeness, a tsarevna, melanomas, an ire…globigerinas, a mon, a leman, \
             Vera, Stassen, Evita, Genaro, Donavon, a villein, a delight, \
             Nica, Janik, Fulahs, Komarek, a ter, a canal, Panama.",
        ),
        ("mostly_special", "!!!@@@###$$$%%%^^^&&&***((("),
    ];

    for (name, input) in test_cases {
        let input_owned = input.to_string();

        // Skip is_palindrome_1 for long inputs - it's O(n²) and will take forever
        if !name.starts_with("long") {
            group.bench_function(format!("is_palindrome_1_{}", name), |b| {
                b.iter(|| is_palindrome_1(black_box(input_owned.clone())))
            });
        }

        group.bench_function(format!("is_palindrome_2_{}", name), |b| {
            b.iter(|| is_palindrome_2(black_box(input_owned.clone())))
        });

        group.bench_function(format!("is_palindrome_3_{}", name), |b| {
            b.iter(|| is_palindrome_3(black_box(input_owned.clone())))
        });

        group.bench_function(format!("is_palindrome_bytes_{}", name), |b| {
            b.iter(|| is_palindrome_4(black_box(input_owned.clone())))
        });

        group.bench_function(format!("is_palindrome_iterators_{}", name), |b| {
            b.iter(|| is_palindrome_iterators(black_box(input_owned.clone())))
        });

        group.bench_function(format!("is_palindrome_bytes_vec_{}", name), |b| {
            b.iter(|| is_palindrome_bytes_vec(black_box(input_owned.clone())))
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_palindrome);
criterion_main!(benches);
