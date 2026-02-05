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

// 0ms | 2.20MB - 84.96%
pub fn search_insert_2(nums: Vec<i32>, target: i32) -> i32 {
    match nums.binary_search(&target) {
        Ok(i) => i as i32,
        Err(i) => i as i32,
    }
}

// 0ms | 2.14MB - 86.96%
pub fn search_insert_3(nums: Vec<i32>, target: i32) -> i32 {
    let mut s = 0;
    let mut e = nums.len();

    while e > s {
        let i = (s + e) / 2;

        if target == nums[i] {
            return i as i32;
        } else if target > nums[i] {
            s = i + 1;
        } else {
            e = i;
        }
    }

    s as _
}

// 0ms | 2.03MB - 99.5%
// idiomatic solution | with hint
pub fn search_insert_4(nums: Vec<i32>, target: i32) -> i32 {
    let mut lo = 0;
    let mut hi = nums.len();

    while hi > lo {
        let mid = lo + (hi - lo) / 2;

        if nums[mid] < target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }

    lo as _
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        search_insert_1,
        search_insert_2,
        search_insert_3,
        search_insert_4,
    );

    fn run_test(target_fn: fn(Vec<i32>, i32) -> i32) {
        vec![
            (vec![1], 0, 0),
            (vec![1], 2, 1),
            (vec![1, 2, 3, 6], 2, 1),
            (vec![1, 3, 4, 5, 6], 5, 3),
            (vec![1, 3, 5, 6], 2, 1),
            (vec![1, 3, 5, 7], 6, 3),
        ]
        .into_iter()
        .for_each(|(nums, target, expected)| {
            let name = format!("{nums:?} {target}");
            let actual = target_fn(nums, target);
            assert_that!(actual).named(&name).is_equal_to(expected);
        });
    }
}
