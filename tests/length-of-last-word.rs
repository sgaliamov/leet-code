//! https://leetcode.com/problems/length-of-last-word/
//!
//! Given a string s consisting of words and spaces, return the length of the last word in the string.
//! A word is a maximal substring consisting of non-space characters only.
//!
//! Constraints:
//! - 1 <= s.length <= 10^4
//! - s consists of only English letters and spaces ' '.
//! - There will be at least one word in s.

// 0ms | 2.14 MB - 62.56%
pub fn length_of_last_word_1(s: String) -> i32 {
    s.split_whitespace().last().unwrap().len() as i32
}

// 0ms | 2.18 MB - 60.33%
pub fn length_of_last_word_2(s: String) -> i32 {
    let bytes = s.as_bytes();
    let mut len = 0;

    for i in (0..bytes.len()).rev() {
        if bytes[i] == b' ' {
            if len == 0 {
                continue;
            } else {
                break;
            }
        } else {
            len += 1;
        }
    }

    len
}

// 0ms | 2.07MB - 97.62%
pub fn length_of_last_word_3(s: String) -> i32 {
    let bytes = s.as_bytes();
    let mut len = 0;

    for i in (0..bytes.len()).rev() {
        if bytes[i] != b' ' {
            len += 1;
        } else if len != 0 {
            return len;
        }
    }

    len
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        length_of_last_word_1,
        length_of_last_word_2,
        length_of_last_word_3,
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
