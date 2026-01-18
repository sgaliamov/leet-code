//! https://leetcode.com/problems/is-subsequence
//!
//! Given two strings s and t, return true if s is a subsequence of t, or false otherwise.
//! A subsequence of a string is a new string that is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (i.e., "ace" is a subsequence of "abcde" while "aec" is not).

pub fn is_subsequence(s: String, t: String) -> bool {
    let mut i = 0;
    let mut j = 0;
    let s = s.as_bytes();
    let t = t.as_bytes();

    while j < t.len() && i < s.len() {
        if s[i] == t[j] {
            i += 1;
            j += 1;
        } else {
            j += 1;
        }
    }

    i == s.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn test() {
        run_test(is_subsequence);
    }

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
