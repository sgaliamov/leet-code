//! https://leetcode.com/problems/valid-parentheses
//!
//! Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
//! An input string is valid if:
//! - Open brackets must be closed by the same type of brackets.
//! - Open brackets must be closed in the correct order.
//! - Every close bracket has a corresponding open bracket of the same type.
//!
//! Constraints:
//! 1 <= s.length <= 10^4
//! s consists of parentheses only '()[]{}'.

// 100-0/78-2.19
pub fn is_valid_1(s: String) -> bool {
    let mut stack = vec![];
    for c in s.into_bytes() {
        match c {
            b'(' | b'[' | b'{' => stack.push(c),

            _ => {
                let Some(l) = stack.pop() else {
                    return false;
                };

                let r = match c {
                    b')' => b'(',
                    b']' => b'[',
                    b'}' => b'{',
                    _ => unreachable!(),
                };

                if r != l {
                    return false;
                }
            }
        }
    }

    stack.is_empty()
}

// 100/99/2.03
pub fn is_valid_2(s: String) -> bool {
    let mut k = 0;
    let mut chars = s.into_bytes();

    for i in 0..chars.len() {
        let c = chars[i];

        if let b'(' | b'[' | b'{' = c {
            chars[k] = c;
            k += 1;
        } else {
            if k == 0 {
                return false;
            }

            let actual = chars[k - 1];
            k -= 1;

            let expected = match c {
                b')' => b'(',
                b']' => b'[',
                b'}' => b'{',
                _ => unreachable!(),
            };

            if expected != actual {
                return false;
            }
        }
    }

    k == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        is_valid_1,
        is_valid_2,
    );

    fn run_test(target: fn(String) -> bool) {
        vec![
            ("()[]{}", true),
            ("(", false),
            ("()", true),
            ("]", false),
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
