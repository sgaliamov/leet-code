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

pub fn remove_duplicates_1(nums: &mut Vec<i32>) -> usize {
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
            nums[p] = v;
            p += 1;
        }
    }

    p
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        remove_duplicates_1,
    );

    fn run_test(target: fn(&mut Vec<i32>) -> usize) {
        let cases = vec![
            (
                vec![0, 0, 1, 1, 1, 1, 2, 3, 3],
                7,
                vec![0, 0, 1, 1, 2, 3, 3],
            ),
            (vec![1, 1, 1, 2, 2, 3], 5, vec![1, 1, 2, 2, 3]),
        ];
        for (mut input, expected_k, expected_nums) in cases {
            let name = format!(
                "input={:?}, expected_k={}, expected_nums={:?}",
                input, expected_k, expected_nums
            );

            let actual_k = target(&mut input);

            assert_that!(input[..actual_k].to_vec())
                .named(&name)
                .is_equal_to(expected_nums[..].to_vec());
            assert_that!(actual_k).named(&name).is_equal_to(expected_k);
        }
    }
}
