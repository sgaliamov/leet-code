//! https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number
//!
//! Given the array nums, for each nums[i] find out how many numbers in the array are smaller than it.
//! That is, for each nums[i] you have to count the number of valid j's such that j != i and nums[j] < nums[i].
//! Return the answer in an array.

pub fn smaller_numbers_than_current_1(nums: Vec<i32>) -> Vec<i32> {
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
        smaller_numbers_than_current_1,
    );

    fn run_test(target: fn(Vec<i32>) -> Vec<i32>) {
        vec![(vec![1, 3, 2, 1], vec![1, 3, 2, 1, 1, 3, 2, 1])]
            .into_iter()
            .for_each(|(nums, expected)| {
                let actual = target(nums);
                let name = format!("{expected:?}");

                assert_that!(actual).named(&name).is_equal_to(&expected);
            });
    }
}
