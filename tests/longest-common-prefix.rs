#![allow(clippy::needless_range_loop)]

//! https://leetcode.com/problems/longest-common-prefix/
//!
//! Write a function to find the longest common prefix string amongst an array of strings.
//! If there is no common prefix, return an empty string "".
//!
//! Constraints:
//! - 1 <= strs.length <= 200
//! - 0 <= strs[i].length <= 200
//! - strs[i] consists of only lowercase English letters if it is non-empty.

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut chars = vec![];

    for col in 0..strs[0].len() {
        let first = strs[0].as_bytes()[col];

        for row in 1..strs.len() {
            if strs[row].len() <= col || strs[row].as_bytes()[col] != first {
                return String::from_iter(chars);
            }
        }

        chars.push(first as char);
    }

    String::from_iter(chars)
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        longest_common_prefix,
    );

    fn run_test(target: fn(Vec<String>) -> String) {
        let cases = vec![
            (
                vec!["flower", "flow", "flight"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                "fl".to_string(),
            ),
            (
                vec!["dog", "racecar", "car"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                "".to_string(),
            ),
        ];
        for (input, expected) in cases {
            let name = format!("input: {:?}", input);
            let actual = target(input);
            assert_that!(actual).named(&name).is_equal_to(expected);
        }
    }
}
