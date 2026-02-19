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

// 8ms - 79.77% | 6.77MB - 22.96%
pub fn contains_nearby_duplicate_2(nums: Vec<i32>, k: i32) -> bool {
    use std::collections::HashMap;
    use std::hash::{BuildHasherDefault, DefaultHasher};

    let mut map: HashMap<_, _, _> =
        HashMap::with_hasher(BuildHasherDefault::<DefaultHasher>::default());
    let k = k as usize;

    for i in 0..nums.len() {
        let n = nums[i];

        if let Some(j) = map.insert(n, i)
            && i - j <= k
        {
            return true;
        }
    }

    false
}

// 1044ms - 5.06% | 3.53MB - 83.27%
pub fn contains_nearby_duplicate_3(nums: Vec<i32>, k: i32) -> bool {
    let k = k as usize;
    let mut set = std::collections::VecDeque::with_capacity(k + 1);

    for i in 0..nums.len() {
        let n = nums[i];

        if set.contains(&n) {
            return true;
        }

        set.push_back(n);
        if set.len() > k {
            set.pop_front();
        }
    }

    false
}

// 1047ms - 5.06% | 3.02MB - 97.62%
pub fn contains_nearby_duplicate_4(nums: Vec<i32>, k: i32) -> bool {
    for i in 0..nums.len() {
        let n = nums[i];
        let j = i.saturating_sub(k as usize);
        if nums[j..i].contains(&n) {
            return true;
        }
    }
    false
}

// 3ms - 96.89% | 3.58MB - 83.27%
pub fn contains_nearby_duplicate_5(nums: Vec<i32>, k: i32) -> bool {
    use std::collections::HashSet;
    use std::hash::{BuildHasherDefault, DefaultHasher};

    let k = k as usize;
    let mut set = HashSet::with_hasher(BuildHasherDefault::<DefaultHasher>::default());

    for i in 0..nums.len() {
        let n = nums[i];

        if !set.insert(n) {
            return true;
        }

        if k <= i {
            set.remove(&nums[i - k]);
        }
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
        contains_nearby_duplicate_2,
        contains_nearby_duplicate_3,
        contains_nearby_duplicate_4,
        contains_nearby_duplicate_5,
    );

    fn run_test(target: fn(Vec<i32>, i32) -> bool) {
        vec![
            (vec![1, 2, 3, 1, 2, 3], 2, false),
            (vec![7, 9, 6, 1, 2, 3, 1, 2, 3], 2, false),
            (vec![1, 1], 0, false),
            (vec![1, 0, 1, 1], 1, true),
            (vec![1, 2, 3, 1], 3, true),
        ]
        .into_iter()
        .for_each(|(nums, k, expected)| {
            let name = format!("nums={:?}, k={}", nums, k);
            let actual = target(nums, k);
            assert_that!(actual).named(&name).is_equal_to(expected);
        });
    }
}
