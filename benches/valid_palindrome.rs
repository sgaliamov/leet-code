use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

// Original implementations
fn is_palindrome_1(s: String) -> bool {
    let mut i = 0_usize;
    let mut j = s.len() - 1;
    let mut letters = std::collections::HashSet::new();
    letters.extend('a'..='z');
    letters.extend('0'..='9');

    while i < j {
        let a = s.chars().nth(i).unwrap().to_lowercase().next().unwrap();
        if !letters.contains(&a) {
            i += 1;
            continue;
        }

        let b = s.chars().nth(j).unwrap().to_lowercase().next().unwrap();
        if !letters.contains(&b) {
            j -= 1;
            continue;
        }

        if a != b {
            return false;
        }

        i += 1;
        j -= 1;
    }

    true
}

fn is_palindrome_2(s: String) -> bool {
    use itertools::Itertools;
    let mut i = 0_usize;
    let mut j = s.len() - 1;
    let chars = s.chars().collect_vec();

    while i < j {
        let a = chars[i];
        if !a.is_alphanumeric() {
            i += 1;
            continue;
        }

        let b = chars[j];
        if !b.is_alphanumeric() {
            j -= 1;
            continue;
        }

        if !a.eq_ignore_ascii_case(&b) {
            return false;
        }

        i += 1;
        j -= 1;
    }

    true
}

fn is_palindrome_3(s: String) -> bool {
    let chars: Vec<_> = s
        .chars()
        .filter(|x| x.is_alphanumeric())
        .map(|x| x.to_ascii_lowercase())
        .collect();

    if chars.is_empty() {
        return true;
    }

    let mut i = 0_usize;
    let mut j = chars.len() - 1;

    while i < j {
        let a = chars[i];
        let b = chars[j];

        if a != b {
            return false;
        }

        i += 1;
        j -= 1;
    }

    true
}

// New implementation: byte slice with no allocation
fn is_palindrome_bytes(s: String) -> bool {
    let bytes = s.as_bytes();
    let mut i = 0;
    let mut j = bytes.len();

    while i < j {
        j -= 1;

        let a = bytes[i];
        if !a.is_ascii_alphanumeric() {
            i += 1;
            continue;
        }

        let b = bytes[j];
        if !b.is_ascii_alphanumeric() {
            continue;
        }

        if !a.eq_ignore_ascii_case(&b) {
            return false;
        }

        i += 1;
    }

    true
}

// New implementation: Iterator-based two-pointer approach
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

// New implementation: Collect filtered bytes first (like version 3 but with bytes)
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
            b.iter(|| is_palindrome_bytes(black_box(input_owned.clone())))
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
