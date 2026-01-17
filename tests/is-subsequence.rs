//! https://leetcode.com/problems/is-subsequence
//!
//! Given two strings s and t, return true if s is a subsequence of t, or false otherwise.
//! A subsequence of a string is a new string that is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (i.e., "ace" is a subsequence of "abcde" while "aec" is not).

pub fn is_subsequence(s: String, t: String) -> bool {
    true
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
        vec![(vec![1, 3, 2, 1], vec![1, 3, 2, 1, 1, 3, 2, 1])]
            .into_iter()
            .for_each(|(nums, expected)| {
                let actual = target(nums);
                let name = format!("{expected:?}");

                assert_that!(actual).named(&name).is_equal_to(&expected);
            });
    }
}
