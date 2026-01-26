//! https://leetcode.com/problems/concatenation-of-array
//!
//! Given an integer array nums of length n, you want to create an array ans of length 2n where ans[i] == nums[i] and ans[i + n] == nums[i] for 0 <= i < n (0-indexed).
//! Specifically, ans is the concatenation of two nums arrays.
//! Return the array ans.

/// Pre-allocates capacity and extends twice.
///
/// Time: O(n) - two linear passes to copy elements
/// Space: O(n) - allocates new vector of size 2n
///
/// Benchmarks (n=elements):
/// - small (n=4): ~52 ns
/// - medium (n=10): ~61 ns
/// - large (n=100): ~145 ns
pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::with_capacity(nums.len() * 2);
    ans.extend(&nums);
    ans.extend(&nums);
    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        get_concatenation,
    );

    fn run_test(target: fn(Vec<i32>) -> Vec<i32>) {
        vec![(vec![1, 3, 2, 1], vec![1, 3, 2, 1, 1, 3, 2, 1])]
            .into_iter()
            .for_each(|(nums, expected)| {
                let name = format!("{nums:?} {expected:?}");
                let actual = target(nums);

                assert_that!(actual).named(&name).is_equal_to(&expected);
            });
    }
}
