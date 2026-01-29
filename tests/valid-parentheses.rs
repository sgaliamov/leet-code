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

use std::{alloc::Layout, mem};

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

pub fn is_valid_2(s: String) -> bool {
    // if s.len() % 2 != 0 {
    //     return false;
    // }

    // let mut stack = vec![0_u64; (s.len() / 2 / 64).max(1)];
    // let mut cursor = 0;
    // const STEP: usize = 8;
    // for c in s.into_bytes() {
    //     let tbit = match c {
    //         b'(' => 0x010,
    //         b')' => 0x011,
    //         b'[' => 0x100,
    //         b']' => 0x101,
    //         b'{' => 0x110,
    //         b'}' => 0x111,
    //         _ => unreachable!(),
    //     };
    //     let bucket = cursor / 64;
    //     let stack = stack[bucket];
    //     let i = cursor % STEP;
    //     stack |= 1 << i;
    // }

    false
}

// 100/99/2.06
pub fn is_valid_3(s: String) -> bool {
    let mut k: i32 = -1;
    let mut bytes = s.into_bytes();

    for i in 0..bytes.len() {
        let c = bytes[i];
        match c {
            b'(' | b'[' | b'{' => {
                k += 1;
                bytes[k as usize] = c;
            }

            _ => {
                if k == -1 {
                    return false;
                }

                let l = bytes[k as usize];
                k -= 1;

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

    k == -1
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        is_valid_1,
        is_valid_3,
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
