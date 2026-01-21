//! https://leetcode.com/problems/isomorphic-strings
//!
//! Given two strings s and t, determine if they are isomorphic.
//! Two strings s and t are isomorphic if the characters in s can be replaced to get t.
//! All occurrences of a character must be replaced with another character while preserving the order of characters. No two characters may map to the same character, but a character may map to itself.

pub fn is_isomorphic_1(r: String, t: String) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        is_isomorphic_1,
    );

    fn run_test(target: fn(String, String) -> bool) {
        vec![
            ("egg", "add", true),
            ("foo", "bar", false),
            ("paper", "title", true),
        ]
        .into_iter()
        .for_each(|(s, t, expected)| {
            let name = format!("{s} {t}");
            let actual = target(s.to_string(), t.to_string());
            assert_that!(actual).named(&name).is_equal_to(expected);
        });
    }
}
