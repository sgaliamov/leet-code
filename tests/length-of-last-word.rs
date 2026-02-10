//! https://leetcode.com/problems/length-of-last-word/
//!
//! Given a string s consisting of words and spaces, return the length of the last word in the string.
//!
//! A word is a maximal substring consisting of non-space characters only.
//!
//! Example 1:
//! Input: s = "Hello World"
//! Output: 5
//! Explanation: The last word is "World" with length 5.
//!
//! Example 2:
//! Input: s = "   fly me   to   the moon  "
//! Output: 4
//! Explanation: The last word is "moon" with length 4.
//!
//! Example 3:
//! Input: s = "luffy is still joyboy"
//! Output: 6
//! Explanation: The last word is "joyboy" with length 6.
//!
//! Constraints:
//! - 1 <= s.length <= 10^4
//! - s consists of only English letters and spaces ' '.
//! - There will be at least one word in s.

// 0ms | 2.14 MB - 62.56%
pub fn length_of_last_word(s: String) -> i32 {
    s.split_whitespace().last().unwrap().len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        length_of_last_word,
    );

    fn run_test(target: fn(String) -> i32) {
        vec![
            ("Hello World".to_string(), 5),                 // last word is "World"
            ("   fly me   to   the moon  ".to_string(), 4), // last word is "moon"
            ("luffy is still joyboy".to_string(), 6),       // last word is "joyboy"
        ]
        .into_iter()
        .for_each(|(s, expected)| {
            let name = format!("s=\"{}\"", s);
            let actual = target(s);
            assert_that!(actual).named(&name).is_equal_to(expected);
        });
    }
}
