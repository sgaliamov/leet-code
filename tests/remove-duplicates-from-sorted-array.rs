#![allow(clippy::ptr_arg)]

//! https://leetcode.com/problems/remove-duplicates-from-sorted-array
//!
//! Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once. The relative order of the elements should be kept the same.
//! Consider the number of unique elements in nums to be k​​​​​​​​​​​​​​. After removing duplicates, return the number of unique elements k.
//! The first k elements of nums should contain the unique numbers in sorted order. The remaining elements beyond index k - 1 can be ignored.

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut k = 0;

    if nums.is_empty() {
        return 0;
    }

    let mut i = 0;
    while i < nums.len() {
        if nums[i] != nums[k] {
            k += 1;
            nums[k] = nums[i];
        }

        i += 1;
    }

    (k + 1) as i32
}

pub fn remove_duplicates_2(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use spectral::prelude::*;

    #[test]
    fn test() {
        run_test(remove_duplicates);
    }

    #[test]
    fn test_2() {
        run_test(remove_duplicates_2);
    }

    fn run_test(target: fn(&mut Vec<i32>) -> i32) {
        vec![
            vec![1, 1, 2],
            vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4],
            vec![0],
            vec![],
        ]
        .into_iter()
        .for_each(|mut nums| {
            let expected = nums.iter().dedup().cloned().collect_vec();
            let actual = target(&mut nums) as usize;
            let nums = nums.into_iter().take(actual).collect_vec();
            let name = format!("{:?}", &expected);

            assert_that!(actual)
                .named(&name)
                .is_equal_to(expected.len());
            assert_that!(nums).named(&name).is_equal_to(expected);
        });
    }
}
