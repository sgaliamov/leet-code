#![allow(clippy::ptr_arg)]

//! https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/
//!
//! Given an integer array nums sorted in non-decreasing order, remove some duplicates in-place such that each unique element appears at most twice. The relative order of the elements should be kept the same.
//! Since it is impossible to change the length of the array in some languages, you must instead have the result be placed in the first part of the array nums. More formally, if there are k elements after removing the duplicates, then the first k elements of nums should hold the final result. It does not matter what you leave beyond the first k elements.
//! Return k after placing the final result in the first k slots of nums.
//! Do not allocate extra space for another array. You must do this by modifying the input array in-place with O(1) extra memory.
//!
//! Custom Judge:
//!
//! The judge will test your solution with the following code:
//!
//! ```
//! int[] nums = [...]; // Input array
//! int[] expectedNums = [...]; // The expected answer with correct length
//! int k = removeDuplicates(nums); // Calls your implementation
//! assert k == expectedNums.length;
//! for (int i = 0; i < k; i++) {
//!     assert nums[i] == expectedNums[i];
//! }
//! ```
//! If all assertions pass, then your solution will be accepted.
//!
//! Constraints:
//! - 1 <= nums.length <= 3 * 10^4
//! - -10^4 <= nums[i] <= 10^4
//! - nums is sorted in non-decreasing order.

// 0ms | 2.16MB - 97.27%
// Bench: 550ns (large), 490ns (worst case)
pub fn remove_duplicates_1(nums: &mut Vec<i32>) -> i32 {
    let mut p = 1;
    let mut v = nums[0];
    let mut cnt = 1;

    for i in 1..nums.len() {
        if nums[i] == v {
            cnt += 1;
        } else {
            cnt = 1;
            v = nums[i];
        }

        if cnt <= 2 {
            if nums[p] != v {
                nums[p] = v;
            }
            p += 1;
        }
    }

    p as i32
}

// 3ms - 70.91% | 2.35MB - 19.55%
// Bench: 654ns (large), 602ns (worst case)
pub fn remove_duplicates_2(nums: &mut Vec<i32>) -> i32 {
    if nums.len() <= 2 {
        return nums.len() as i32;
    }

    let mut p = 2;
    for i in 2..nums.len() {
        if nums[p - 2] == nums[p - 1] {
            if nums[p - 1] != nums[i] {
                nums[p] = nums[i];
                p += 1;
            }
        } else {
            nums[p] = nums[i];
            p += 1;
        }
    }

    p as i32
}

// 0ms | 2.29MB - 65%
// Bench: 523ns (large), 477ns (worst case) - fastest
pub fn remove_duplicates_3(nums: &mut Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return nums.len() as i32;
    }

    let mut p = 2;
    for i in 2..nums.len() {
        if nums[p - 2] != nums[i] {
            if nums[p] != nums[i] {
                nums[p] = nums[i];
            }
            p += 1;
        }
    }

    p as i32
}

// Bench: 381ns (large), 350ns (worst case) - fastest with retain
pub fn remove_duplicates_4(nums: &mut Vec<i32>) -> i32 {
    let mut p = (-1, 0);

    nums.retain(|&n| {
        let res = p.0 == p.1 && p.1 == n;
        p.1 = p.0;
        p.0 = n;
        !res
    });

    nums.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        remove_duplicates_1,
        remove_duplicates_2,
        remove_duplicates_3,
        remove_duplicates_4,
    );

    fn run_test(target: fn(&mut Vec<i32>) -> i32) {
        let cases = vec![
            (
                vec![0, 0, 1, 1, 1, 1, 2, 3, 3],
                7,
                vec![0, 0, 1, 1, 2, 3, 3],
            ),
            (vec![1, 1, 1, 2, 2, 3], 5, vec![1, 1, 2, 2, 3]),
            (vec![1, 2, 3], 3, vec![1, 2, 3]),
            (vec![1, 2, 2, 2, 3], 4, vec![1, 2, 2, 3]),
            (vec![1, 1, 1, 2, 2, 3, 3, 3, 3], 6, vec![1, 1, 2, 2, 3, 3]),
            (vec![1], 1, vec![1]),
        ];
        for (mut input, expected_k, expected_nums) in cases {
            let name = format!(
                "input={:?}, expected_k={}, expected_nums={:?}",
                input, expected_k, expected_nums
            );

            let actual_k = target(&mut input) as usize;

            assert_that!(input[..actual_k].to_vec())
                .named(&name)
                .is_equal_to(expected_nums[..].to_vec());
            assert_that!(actual_k).named(&name).is_equal_to(expected_k);
        }
    }
}
