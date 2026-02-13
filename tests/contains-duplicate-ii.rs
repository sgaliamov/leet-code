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

pub fn contains_nearby_duplicate_1(nums: Vec<i32>, k: i32) -> bool {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    let k = k as usize;

    for i in 0..nums.len() {
        let n = nums[i];

        let e = map.entry(n);
        let j = *e.or_insert(i);

        if i - j <= k && i != j {
            return true;
        }
        map.entry(n).and_modify(|e| *e = i);
    }

    false
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
