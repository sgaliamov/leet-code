//! https://leetcode.com/problems/valid-palindrome
//!
//! A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.
//! Given a string s, return true if it is a palindrome, or false otherwise.

/// correct but slow, because of `nth`
pub fn is_palindrome_1(s: String) -> bool {
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

/// still slow
pub fn is_palindrome_2(s: String) -> bool {
    use itertools::Itertools;
    let chars = s.chars().collect_vec();

    if chars.is_empty() {
        return true;
    }

    let mut i = 0_usize;
    let mut j = chars.len() - 1;

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

/// passed 100% runtime test.
pub fn is_palindrome_3(s: String) -> bool {
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

/// Two-pointer approach on byte slice - fastest implementation.
///
/// Time: O(n) - single pass with two pointers moving toward center
/// Space: O(1) - only uses indices, no allocations
///
/// Benchmarks:
/// - short_true: ~130 ns
/// - short_false: ~99 ns
/// - medium_true: ~80 ns
/// - medium_false: ~82 ns
/// - long_true: ~2.1 µs
/// - long_false: ~440 ns
/// - mostly_special: ~82 ns
pub fn is_palindrome_4(s: String) -> bool {
    let bytes = s.as_bytes();
    let mut i = 0;
    let mut j = bytes.len() - 1;

    while i < j {
        let a = bytes[i];
        if !a.is_ascii_alphanumeric() {
            i += 1;
            continue;
        }

        let b = bytes[j];
        if !b.is_ascii_alphanumeric() {
            j -= 1;

            continue;
        }

        if !a.eq_ignore_ascii_case(&b) {
            return false;
        }

        j -= 1;
        i += 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        is_palindrome_1,
        is_palindrome_2,
        is_palindrome_3,
        is_palindrome_4,
    );

    fn run_test(target: fn(String) -> bool) {
        vec![
            ("A man, a plan, a canal: Panama", true),
            ("race a car", false),
            ("0P", false),
            (" ", true),
            (
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
                true,
            ),
        ]
        .into_iter()
        .for_each(|(str, expected)| {
            let actual = target(str.to_string());
            assert_that!(actual).named(str).is_equal_to(expected);
        });
    }
}
