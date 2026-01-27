//! https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array
//!
//! Given an array nums of n integers where nums[i] is in the range [1, n], return an array of all the integers in the range [1, n] that do not appear in nums.
//!
//! Example 1:
//! Input: nums = [4,3,2,7,8,2,3,1]
//! Output: [5,6]
//!
//! Example 2:
//! Input: nums = [1,1]
//! Output: [2]
//!
//! Constraints:
//! n == nums.length
//! 1 <= n <= 105
//! 1 <= nums[i] <= n
//! Follow up: Could you do it without extra space and in O(n) runtime? You may assume the returned list does not count as extra space.

pub fn find_disappeared_numbers_1(nums: Vec<i32>) -> Vec<i32> {
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
        find_disappeared_numbers_1,
    );

    fn run_test(target: fn(Vec<i32>) -> Vec<i32>) {
        vec![
            (vec![4, 3, 2, 7, 8, 2, 3, 1], vec![5, 6]),
            (vec![1, 2, 3], vec![]),
            (vec![1, 1], vec![2]),
        ]
        .into_iter()
        .for_each(|(nums, expected)| {
            let name = format!("{nums:?} {expected:?}");
            let actual = target(nums);

            assert_that!(actual).named(&name).is_equal_to(&expected);
        });
    }
}
