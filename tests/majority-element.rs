//! <https://leetcode.com/problems/majority-element/>
//!
//! Given an array nums of size n, return the majority element.
//!
//! The majority element is the element that appears more than ⌊n / 2⌋ times.
//! You may assume that the majority element always exists in the array.
//!
//! Constraints:
//! - n == nums.length
//! - 1 <= n <= 5 * 10^4
//! - -10^9 <= nums[i] <= 10^9
//! - The input is generated such that a majority element will exist in the array.
//!
//! Follow-up: Could you solve the problem in linear time and in O(1) space?

// 3ms - 11.90% | 2.44MB - 44.05%
pub fn majority_element_1(nums: Vec<i32>) -> i32 {
    nums.into_iter()
        .fold(
            std::collections::HashMap::<i32, u32>::new(),
            |mut map, n| {
                *map.entry(n).or_default() += 1;
                map
            },
        )
        .into_iter()
        .max_by_key(|&x| x.1)
        .unwrap_or_default()
        .0
}

// 0ms | 2.27MB - 100%
// The Boyer-Moore voting algorithm
pub fn majority_element_2(nums: Vec<i32>) -> i32 {
    let mut candidate = 0;
    let mut counter = 0;

    for n in nums {
        if counter == 0 {
            candidate = n;
        }

        if n == candidate {
            counter += 1;
        } else {
            counter -= 1;
        }
    }

    candidate
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        majority_element_1,
        majority_element_2,
    );

    fn run_test(target: fn(Vec<i32>) -> i32) {
        vec![
            (vec![3, 2, 3], 3),             // Example 1: single majority element
            (vec![2, 2, 1, 1, 1, 2, 2], 2), // Example 2: majority with multiple occurrences
        ]
        .into_iter()
        .for_each(|(nums, expected)| {
            let name = format!("majority_element({:?}) should be {}", nums, expected);
            let actual = target(nums);
            assert_that!(actual).named(&name).is_equal_to(expected);
        });
    }
}
