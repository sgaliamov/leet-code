#![allow(clippy::needless_range_loop)]

//! https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/
//!
//! Given two strings needle and haystack, return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.
//!
//! Constraints:
//! - 1 <= haystack.length, needle.length <= 10^4
//! - haystack and needle consist of only lowercase English characters.

pub fn str_str(haystack: String, needle: String) -> i32 {
    let haystack = haystack.as_bytes();
    let needle = needle.into_bytes();
    let mut i = 0;

    while i < haystack.len() {
        let mut j = 0;

        while j < needle.len() && i < haystack.len() {
            if haystack[i] != needle[j] {
                break;
            }

            j += 1;
            i += 1;
        }

        if j == needle.len() {
            return (i - j) as i32;
        }

        i += 1;
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        str_str,
    );

    fn run_test(target: fn(String, String) -> i32) {
        let cases = vec![
            ("mississippi", "issip", 4),
            ("leetcode", "leeto", -1),
            ("leet", "leetcode", -1),
            ("notsadbutsad", "sad", 3),
            ("sadbutsad", "sad", 0),
        ];
        for (haystack, needle, expected) in cases {
            let name = format!("haystack='{}', needle='{}'", haystack, needle);
            let actual = target(haystack.to_string(), needle.to_string());
            assert_that!(actual).named(&name).is_equal_to(expected);
        }
    }
}
