//! https://leetcode.com/problems/search-insert-position
//!
//! Given a sorted array of distinct integers and a target value, return the index if the target is found.
//! If not, return the index where it would be if it were inserted in order.
//! You must write an algorithm with O(log n) runtime complexity.
//!
//! Constraints:
//! - 1 <= nums.length <= 10^4
//! - -10^4 <= nums[i] <= 10^4
//! - nums contains distinct values sorted in ascending order.
//! - -10^4 <= target <= 10^4

// 0ms | 2.20MB - 34%
pub fn search_insert_1(nums: Vec<i32>, target: i32) -> i32 {
    match nums.binary_search(&target) {
        Ok(i) => i as i32,
        Err(i) => i as i32,
    }
}

// 0ms | 2.20MB - 84.75%
pub fn search_insert_2(nums: Vec<i32>, target: i32) -> i32 {
    match nums.binary_search(&target) {
        Ok(i) => i as i32,
        Err(i) => i as i32,
    }
}

pub fn search_insert_3(nums: Vec<i32>, target: i32) -> i32 {
    let mut i = nums.len() / 2;

    loop {

        if target < nums[i] {
            i /= 2;
        }


    }

    i as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        search_insert_1,
    );

    fn run_test(target_fn: fn(Vec<i32>, i32) -> i32) {
        vec![
            (vec![1, 3, 5, 6], 5, 2), //
            (vec![1, 3, 5, 6], 2, 1),
        ]
        .into_iter()
        .for_each(|(nums, target, expected)| {
            let name = format!("{nums:?} {target}");
            let actual = target_fn(nums, target);
            assert_that!(actual).named(&name).is_equal_to(expected);
        });
    }
}
