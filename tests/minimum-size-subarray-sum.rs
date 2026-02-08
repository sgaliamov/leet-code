//! https://leetcode.com/problems/minimum-size-subarray-sum/
//!
//! Given an array of positive integers `nums` and a positive integer `target`,
//! return the minimal length of a subarray whose sum is greater than or equal to `target`.
//! If there is no such subarray, return 0 instead.
//!
//! Constraints:
//! - 1 <= target <= 10^9
//! - 1 <= nums.length <= 10^5
//! - 1 <= nums[i] <= 10^4
//!
//! Follow-up: If you have figured out the O(n) solution, try coding another solution
//! of which the time complexity is O(n log(n)).

// 0ms | 2.87MB - 95.88%
pub fn min_subarray_len_1(target: i32, nums: Vec<i32>) -> i32 {
    let mut lo = 0;
    let mut hi = 0;
    let mut min = usize::MAX;
    let mut sum = nums[0];

    loop {
        if sum >= target {
            min = min.min(hi - lo + 1);
            sum -= nums[lo];
            lo += 1;

            if lo == nums.len() {
                break;
            }
        } else if hi == nums.len() {
            break;
        }

        if sum < target {
            hi += 1;
            if hi < nums.len() {
                sum += nums[hi];
            }
        }
    }

    if min == usize::MAX { 0 } else { min as i32 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        min_subarray_len_1,
    );

    fn run_test(target: fn(i32, Vec<i32>) -> i32) {
        vec![
            (7, vec![2, 3, 1, 2, 4, 1], 3),
            (7, vec![2, 3, 1, 2, 4, 3], 2),
            (2, vec![1], 0),
            (4, vec![1, 4, 4], 1),
            (11, vec![1, 1, 1, 1, 1, 1, 1, 1], 0),
            (3, vec![1, 1, 1], 3),
            (1, vec![1], 1),
        ]
        .into_iter()
        .for_each(|(target_val, nums, expected)| {
            let name = format!("target={}, nums={:?}", target_val, nums);
            let actual = target(target_val, nums);
            assert_that!(actual).named(&name).is_equal_to(expected);
        });
    }
}
