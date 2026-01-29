//! https://leetcode.com/problems/two-sum
//!
//! Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
//! You may assume that each input would have exactly one solution, and you may not use the same element twice.
//! You can return the answer in any order.
//!
//! Constraints:
//! - 2 <= nums.length <= 10^4
//! - -10^9 <= nums[i] <= 10^9
//! - -10^9 <= target <= 10^9
//! - Only one valid answer exists.
//!
//! Follow-up: Can you come up with an algorithm that is less than O(n^2) time complexity?

// 100/82/2.28
pub fn two_sum_1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use itertools::Itertools;
    let sorted = nums.iter().sorted_unstable().collect_vec();

    for i in 0..nums.len() {
        let t = target - nums[i];

        if let Ok(r) = sorted.binary_search(&&t) {
            let v = sorted[r];

            if let Some((p, _)) = nums
                .iter()
                .enumerate()
                .find_position(|&(p, x)| x == v && p != i)
            {
                let i = i as i32;
                let p = p as i32;
                return if i > p { vec![p, i] } else { vec![i, p] };
            }
        }
    }

    vec![]
}

pub fn two_sum_2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use itertools::Itertools;
    let sorted = nums
        .iter()
        .enumerate()
        .sorted_unstable_by_key(|x| x.1)
        .collect_vec();

    for i in 0..nums.len() {
        let t = target - nums[i];

        if let Ok(r) = sorted.binary_search_by_key(&&t, |x| x.1) {
            let (p, _) = sorted[r];
            if i == p {
                continue;
            }

            let i = i as i32;
            let p = p as i32;
            return if i > p { vec![p, i] } else { vec![i, p] };
        }
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        two_sum_1,
        two_sum_2,
    );

    fn run_test(target_fn: fn(Vec<i32>, i32) -> Vec<i32>) {
        vec![
            (vec![3, 2, 4], 6, vec![1, 2]),
            (vec![2, 7, 11, 15], 9, vec![0, 1]),
            (vec![3, 3], 6, vec![0, 1]),
        ]
        .into_iter()
        .for_each(|(nums, target, expected)| {
            let name = format!("{nums:?} {target}");
            let actual = target_fn(nums, target);
            assert_that!(actual).named(&name).is_equal_to(expected);
        });
    }
}
