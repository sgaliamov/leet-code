//! https://leetcode.com/problems/evaluate-reverse-polish-notation/
//!
//! You are given an array of strings tokens that represents an arithmetic expression in a Reverse Polish Notation.
//!
//! Evaluate the expression. Return an integer that represents the value of the expression.
//!
//! Note that:
//! - The valid operators are '+', '-', '*', and '/'.
//! - Each operand may be an integer or another expression.
//! - The division between two integers always truncates toward zero.
//! - There will not be any division by zero.
//! - The input represents a valid arithmetic expression in a reverse polish notation.
//! - The answer and all the intermediate calculations can be represented in a 32-bit integer.
//!
//! Constraints:
//! - 1 <= tokens.length <= 10^4
//! - tokens[i] is either an operator: "+", "-", "*", or "/", or an integer in the range [-200, 200].

// 0ms | 2.83MB - 28.22%
pub fn eval_rpn_1(tokens: Vec<String>) -> i32 {
    use std::str::FromStr;
    let mut num_stack = Vec::<i32>::new();

    for t in tokens {
        if let "+" | "-" | "*" | "/" = t.as_str() {
            let b = num_stack.pop().unwrap();
            let a = num_stack.pop().unwrap();

            let c = match t.as_str() {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => a / b,
                _ => unreachable!(),
            };
            num_stack.push(c);
        } else {
            num_stack.push(i32::from_str(t.as_str()).unwrap());
        }
    }

    num_stack.pop().unwrap()
}

pub fn eval_rpn_2(tokens: Vec<String>) -> i32 {
    // keep results only
    let mut res_stack = Vec::<i32>::new();

    for i in 0..tokens.len() {
        let t = tokens[i].as_str();

        if let "+" | "-" | "*" | "/" = t {
            let b = res_stack.pop();
            let a = res_stack.pop();

            let (a, b) = match (a, b) {
                (None, None) => (
                    tokens[i - 2].parse().unwrap(),
                    tokens[i - 1].parse().unwrap(),
                ),
                (None, Some(b)) => (tokens[i - 1].parse().unwrap(), b),
                (Some(a), None) => (a, tokens[i - 1].parse().unwrap()),
                (Some(a), Some(b)) => (a, b),
            };

            let c = match t {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => a / b,
                _ => unreachable!(),
            };
            res_stack.push(c);
        }
    }

    res_stack.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        eval_rpn_1,
        eval_rpn_2,
    );

    fn run_test(target: fn(Vec<String>) -> i32) {
        vec![
            (vec!["2", "1", "+", "3", "*"], 9),  // ((2 + 1) * 3) = 9
            (vec!["4", "13", "5", "/", "+"], 6), // (4 + (13 / 5)) = 6
            (
                vec![
                    "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
                ],
                22,
            ), // ((10 * (6 / ((9 + 3) * -11))) + 17) + 5 = 22
        ]
        .into_iter()
        .for_each(|(tokens, expected)| {
            let tokens_str = tokens
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            let name = format!("eval_rpn([{}])", tokens_str);
            let tokens = tokens.into_iter().map(|s| s.to_string()).collect();
            let actual = target(tokens);
            assert_that!(actual).named(&name).is_equal_to(expected);
        });
    }
}
