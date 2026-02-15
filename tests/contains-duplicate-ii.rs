#![allow(clippy::needless_range_loop)]

//! https://leetcode.com/problems/contains-duplicate-ii/
//!
//! Given an integer array nums and an integer k, return true if there are two
//! distinct indices i and j in the array such that nums[i] == nums[j] and
//! abs(i - j) <= k.
//!
//! Constraints:
//! - 1 <= nums.length <= 10^5
//! - -10^9 <= nums[i] <= 10^9
//! - 0 <= k <= 10^5

// 10ms - 69.35% | 6.71MB - 22.61%
pub fn contains_nearby_duplicate_1(nums: Vec<i32>, k: i32) -> bool {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    let k = k as usize;

    for (i, &n) in nums.iter().enumerate() {
        if let Some(&j) = map.get(&n)
            && i - j <= k
        {
            return true;
        }

        map.insert(n, i);
    }

    false
}

pub fn contains_nearby_duplicate_2(nums: Vec<i32>, k: i32) -> bool {
    todo!("improve")
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        contains_nearby_duplicate_1,
    );

    fn run_test(target: fn(Vec<i32>, i32) -> bool) {
        vec![
            (vec![1, 0, 1, 1], 1, true),        // duplicates within distance 1
            (vec![1, 2, 3, 1, 2, 3], 2, false), // duplicates outside distance 2
            (vec![1, 2, 3, 1], 3, true),        // duplicates within distance 3
        ]
        .into_iter()
        .for_each(|(nums, k, expected)| {
            let name = format!("nums={:?}, k={}", nums, k);
            let actual = target(nums, k);
            assert_that!(actual).named(&name).is_equal_to(expected);
        });
    }
}
