//! https://leetcode.com/problems/valid-anagram
//!
//! Given two strings s and t, return true if t is an anagram of s, and false otherwise.
//!
//! Constraints:
//! - 1 <= s.length, t.length <= 5 * 10^4
//! - s and t consist of lowercase English letters.
//!
//! Follow up: What if the inputs contain Unicode characters? How would you adapt your solution to such a case?

// 0ms | 2.65MB - 11
pub fn is_anagram_1(s: String, t: String) -> bool {
    use itertools::Itertools;
    s.len() == t.len()
        && s.as_bytes()
            .iter()
            .sorted_unstable()
            .zip(t.as_bytes().iter().sorted_unstable())
            .take_while(|(a, b)| a == b)
            .count()
            == s.len()
}

// 0ms | 2.2MB - 99.37%
pub fn is_anagram_2(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    use std::collections::HashMap;

    let mut map = HashMap::new();
    for i in 0..s.len() {
        let c = s.as_bytes()[i];
        *map.entry(c).or_insert(0) += 1;
    }

    for i in 0..t.len() {
        let c = t.as_bytes()[i];
        if let Some(e) = map.get_mut(&c) {
            if e == &0 {
                return false;
            }
            *e -= 1;
        } else {
            return false;
        }
    }

    true
}

// pub fn is_anagram_3(s: String, t: String) -> bool {
//     if s.len() != t.len() {
//         return false;
//     }
//     let mut i = 0;
//     let mut xor = 0;
//     while i < s.len() && i < t.len() {
//         xor ^= s.as_bytes()[i];
//         xor ^= t.as_bytes()[i];
//         i += 1;
//     }
//     xor == 0
// }

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        is_anagram_1,
        is_anagram_2,
    );

    fn run_test(target: fn(String, String) -> bool) {
        vec![
            ("nagaram", "anagram", true),
            ("car", "rat", false),
            ("a", "ab", false),
            ("aa", "bb", false),
            ("aa", "aa", true),
            ("🪄😆", "😆🪄", true),
        ]
        .into_iter()
        .for_each(|(pattern, s, expected)| {
            let name = format!("{pattern} {s}");
            let actual = target(pattern.to_string(), s.to_string());
            assert_that!(actual).named(&name).is_equal_to(expected);
        });
    }
}
