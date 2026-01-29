//! https://leetcode.com/problems/valid-parentheses
//!
//! Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
//! An input string is valid if:
//! - Open brackets must be closed by the same type of brackets.
//! - Open brackets must be closed in the correct order.
//! - Every close bracket has a corresponding open bracket of the same type.
//!
//! Constraints:
//! 1 <= s.length <= 104
//! s consists of parentheses only '()[]{}'.

pub fn is_valid_1(s: String) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        is_valid_1,
    );

    fn run_test(target: fn(String) -> bool) {
        vec![
            ("()", true),
            ("()[]{}", true),
            ("(]", false),
            ("([])", true),
            ("([)]", false),
        ]
        .into_iter()
        .for_each(|(str, expected)| {
            let actual = target(str.to_string());
            assert_that!(actual).named(str).is_equal_to(expected);
        });
    }
}
