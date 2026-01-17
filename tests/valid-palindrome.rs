//! https://leetcode.com/problems/valid-palindrome
//!
//! A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.
//! Given a string s, return true if it is a palindrome, or false otherwise.

/// correct but slow
pub fn is_palindrome_1(s: String) -> bool {
    let mut i = 0_usize;
    let mut j = s.len() - 1;
    let mut letters = std::collections::HashSet::new();
    letters.extend('a'..='z');
    letters.extend('0'..='9');

    while i < j {
        let a = s.chars().nth(i).unwrap().to_lowercase().next().unwrap();
        if !letters.contains(&a) {
            i += 1;
            continue;
        }

        let b = s.chars().nth(j).unwrap().to_lowercase().next().unwrap();
        if !letters.contains(&b) {
            j -= 1;
            continue;
        }

        if a != b {
            return false;
        }

        i += 1;
        j -= 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn test() {
        run_test(is_palindrome_1);
    }

    fn run_test(target: fn(String) -> bool) {
        vec![
            ("A man, a plan, a canal: Panama", true),
            ("race a car", false),
            ("0P", false),
            (" ", true),
        ]
        .into_iter()
        .for_each(|(str, expected)| {
            let actual = target(str.to_string());
            assert_that!(actual).named(str).is_equal_to(expected);
        });
    }
}
