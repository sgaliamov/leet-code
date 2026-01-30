//! https://leetcode.com/problems/build-an-array-with-stack-operations/
//!
//! You are given an integer array target and an integer n.
//!
//! You have an empty stack with the two following operations:
//! - "Push": pushes an integer to the top of the stack.
//! - "Pop": removes the integer on the top of the stack.
//!
//! You also have a stream of the integers in the range [1, n].
//!
//! Use the two stack operations to make the numbers in the stack (from the bottom to the top)
//! equal to target. You should follow the following rules:
//! - If the stream of the integers is not empty, pick the next integer from the stream and push it to the top of the stack.
//! - If the stack is not empty, pop the integer at the top of the stack.
//! - If, at any moment, the elements in the stack (from the bottom to the top) are equal to target,
//!   do not read new integers from the stream and do not do more operations on the stack.
//!
//! Return the stack operations needed to build target following the mentioned rules.
//! If there are multiple valid answers, return any of them.
//!
//! Constraints:
//! - 1 <= target.length <= 100
//! - 1 <= n <= 100
//! - 1 <= target[i] <= n
//! - target is strictly increasing

// 0ms | 2.21MB - 63.86%
pub fn build_array_1(target: Vec<i32>, n: i32) -> Vec<String> {
    let mut stream = 1..=n;
    let mut ops = vec![];

    for t in target {
        for n in stream.by_ref() {
            if n == t {
                ops.push("Push".into());
                break;
            } else {
                ops.push("Push".into());
                ops.push("Pop".into());
            }
        }

        if stream.is_empty() {
            break;
        }
    }

    ops
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    leet_code::solution_tests!(
        run_test:
        build_array_1,
    );

    fn run_test(target: fn(Vec<i32>, i32) -> Vec<String>) {
        let cases = vec![
            (vec![1, 3], 3, vec!["Push", "Push", "Pop", "Push"]), // Read 1, push; Read 2, push & pop; Read 3, push
            (vec![1, 2, 3], 3, vec!["Push", "Push", "Push"]),     // All sequential, just push
            (vec![1, 2], 4, vec!["Push", "Push"]),                // Stop after building target
            (vec![], 5, vec![]),                                   // empty target
        ];

        for (input_target, n, expected) in cases {
            let expected: Vec<String> = expected.into_iter().map(String::from).collect();
            let actual = target(input_target, n);
            assert_that!(actual).is_equal_to(expected);
        }
    }
}
