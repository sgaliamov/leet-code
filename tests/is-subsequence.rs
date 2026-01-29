//! https://leetcode.com/problems/is-subsequence
//!
//! Given two strings s and t, return true if s is a subsequence of t, or false otherwise.
//! A subsequence of a string is a new string that is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (i.e., "ace" is a subsequence of "abcde" while "aec" is not).

// 0ms | 72/2.15
pub fn is_subsequence_1(s: String, t: String) -> bool {
    let s = s.into_bytes();
    let t = t.into_bytes();
    let (mut i, mut j, m, n) = (0, 0, s.len(), t.len());

    while j < n && i < m {
        if s[i] == t[j] {
            i += 1;
            if i == m {
                return true;
            }
        }

        j += 1;
    }

    s.is_empty()
}

// 0ms | 72/2.19
pub fn is_subsequence_2(s: String, t: String) -> bool {
    if s.is_empty() {
        return true;
    }

    let s = s.into_bytes();
    let mut i = 0;

    for c in t.into_bytes() {
        if s[i] == c {
            i += 1;
            if i == s.len() {
                return true;
            }
        }
    }

    false
}

// 0ms | 99/2.08
pub fn is_subsequence_3(s: String, t: String) -> bool {
    if s.is_empty() {
        return true;
    }

    let mut i = 0;

    for j in 0..t.len() {
        if s.as_bytes()[i] == t.as_bytes()[j] {
            i += 1;
            if i == s.len() {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        is_subsequence_1,
        is_subsequence_2,
        is_subsequence_3,
    );

    fn run_test(target: fn(String, String) -> bool) {
        vec![
            ("abc", "ahbgdc", true),
            ("axc", "ahbgdc", false),
            ("", "ahbgdc", true),
            ("", "", true),
            ("arst", "arst", true),
            ("arstg", "", false),
        ]
        .into_iter()
        .for_each(|(s, t, expected)| {
            let actual = target(s.to_string(), t.to_string());
            let name = format!("{s}-{t}");
            assert_that!(actual).named(&name).is_equal_to(expected);
        });
    }
}
