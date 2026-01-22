//! https://leetcode.com/problems/word-pattern
//!
//! Given a pattern and a string s, find if s follows the same pattern.
//! Here follow means a full match, such that there is a bijection between a letter in pattern and a non-empty word in s. Specifically:
//! Each letter in pattern maps to exactly one unique word in s.
//! Each unique word in s maps to exactly one letter in pattern.
//! No two letters map to the same word, and no two words map to the same letter.

/// good runtime, mediocre memory
pub fn word_pattern_1(pattern: String, s: String) -> bool {
    let mut i = 0;
    let pattern = pattern.as_bytes();
    let mut map = [""; 26];

    for s in s.split(' ') {
        if i == pattern.len(){
            return false;
        }

        let pi = (pattern[i] - b'a') as usize;

        if map[pi].is_empty() {
            if map.contains(&s) {
                return false;
            }

            map[pi] = s;
        }

        if map[pi] != s {
            return false;
        }

        i += 1;
    }

    i == pattern.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        word_pattern_1,
    );

    fn run_test(target: fn(String, String) -> bool) {
        vec![
            ("aaa", "aa aa aa aa", false),
            ("abba", "dog cat cat dog", true),
            ("abbaa", "dog cat cat dog", false),
            ("ab", "dog dog", false),
            ("abba", "dog cat cat fish", false),
            ("aaaa", "dog cat cat dog", false),
        ]
        .into_iter()
        .for_each(|(pattern, s, expected)| {
            let name = format!("{pattern} {s}");
            let actual = target(pattern.to_string(), s.to_string());
            assert_that!(actual).named(&name).is_equal_to(expected);
        });
    }
}
