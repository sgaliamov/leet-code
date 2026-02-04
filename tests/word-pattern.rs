//! https://leetcode.com/problems/word-pattern
//!
//! Given a pattern and a string s, find if s follows the same pattern.
//! Here follow means a full match, such that there is a bijection between a letter in pattern and a non-empty word in s. Specifically:
//! Each letter in pattern maps to exactly one unique word in s.
//! Each unique word in s maps to exactly one letter in pattern.
//! No two letters map to the same word, and no two words map to the same letter.

/// 0ms | 2.20MB - 75%
pub fn word_pattern_1(pattern: String, s: String) -> bool {
    let mut i = 0;
    let pattern = pattern.into_bytes();
    let mut map = [""; 26];

    for s in s.split_whitespace() {
        if i == pattern.len() {
            return false;
        }

        let pi = (pattern[i] - b'a') as usize;

        if map[pi].is_empty() && !map.contains(&s) {
            map[pi] = s;
        } else if map[pi] != s {
            return false;
        }

        i += 1;
    }

    i == pattern.len()
}

// 0ms | 33
pub fn word_pattern_2(pattern: String, s: String) -> bool {
    use itertools::Itertools;
    use std::collections::HashMap;
    use std::hash::*;

    let mut pattern = pattern.bytes();
    let mut map: HashMap<u8, &str, _> =
        HashMap::with_hasher(BuildHasherDefault::<DefaultHasher>::default());

    for s in s.split(' ') {
        let Some(c) = pattern.next() else {
            return false;
        };

        if !map.contains_key(&c) && map.values().contains(&s) {
            return false;
        }

        let e = map.entry(c).or_insert(s);

        if e != &s {
            return false;
        }
    }

    pattern.next().is_none()
}

// 0ms | 33
pub fn word_pattern_3(pattern: String, s: String) -> bool {
    use itertools::Itertools;
    use std::collections::HashMap;
    use std::hash::*;

    let mut i = 0;
    let pattern = pattern.into_bytes();
    let capacity = pattern.iter().unique().count();
    let mut map: HashMap<u8, &str, _> =
        HashMap::with_capacity_and_hasher(capacity, BuildHasherDefault::<DefaultHasher>::default());

    for s in s.split_whitespace() {
        if i == pattern.len() {
            return false;
        }

        let c = pattern[i];

        if !map.contains_key(&c) && map.values().contains(&s) {
            return false;
        }

        let e = map.entry(c).or_insert(s);

        if e != &s {
            return false;
        }

        i += 1;
    }

    i == pattern.len()
}

// 0ms | 71
pub fn word_pattern_4(pattern: String, s: String) -> bool {
    let pattern = pattern.into_bytes();
    let mut seen = 0u32; // 32 bits is enough for 26 letters
    for &c in pattern.iter() {
        seen |= 1 << (c - b'a');
    }
    let capacity = seen.count_ones() as usize;
    const NONE: u8 = u8::MAX;
    let mut ps = vec![NONE; capacity];
    let mut cs = vec![""; capacity];

    let mut pi = 0;
    let mut si = 0;
    let mut bi = 0;

    while pi < pattern.len() && bi < s.len() {
        while bi < s.len() {
            if &s[bi..=bi] == " " {
                break;
            }
            bi += 1;
        }
        let str = &s[si..bi];
        bi += 1;
        si = bi;

        let c = pattern[pi];
        pi += 1;

        let mut psi = 0;
        while psi < capacity {
            if ps[psi] == NONE {
                ps[psi] = c;
                break;
            }

            if ps[psi] == c {
                break;
            }

            psi += 1;
        }

        let mut ssi = 0;
        while ssi < capacity {
            if cs[ssi].is_empty() {
                cs[ssi] = str;
                break;
            }

            if cs[ssi] == str {
                break;
            }

            ssi += 1;
        }

        if ssi != psi {
            return false;
        }
    }

    pi == pattern.len() && bi > s.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        word_pattern_1,
        word_pattern_2,
        word_pattern_3,
        word_pattern_4,
    );

    fn run_test(target: fn(String, String) -> bool) {
        vec![
            ("aaa", "aa aa", false),
            ("aaa", "aa aa aa aa", false),
            ("abba", "dog cat cat dog", true),
            ("ab", "dog dog", false),
            ("aa", "dog cat", false),
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
