//! https://leetcode.com/problems/valid-anagram
//!
//! Given two strings s and t, return true if t is an anagram of s, and false otherwise.

// 0ms | 11
pub fn is_anagram_1(s: String, t: String) -> bool {
    use itertools::Itertools;
    s.len() == t.len()
        && s.chars()
            .sorted_unstable()
            .zip(t.chars().sorted_unstable())
            .take_while(|(a, b)| a == b)
            .count()
            == s.len()
}

pub fn is_anagram_2(s: String, t: String) -> bool {
    todo!("use hash map")
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        is_anagram_1,
    );

    fn run_test(target: fn(String, String) -> bool) {
        vec![
            ("nagaram", "anagram", true), //
            ("car", "rat", false),
        ]
        .into_iter()
        .for_each(|(pattern, s, expected)| {
            let name = format!("{pattern} {s}");
            let actual = target(pattern.to_string(), s.to_string());
            assert_that!(actual).named(&name).is_equal_to(expected);
        });
    }
}
