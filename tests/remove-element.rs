//! https://leetcode.com/problems/remove-element

use spectral::prelude::*;

/// Given an integer array nums and an integer val, remove all occurrences of val in nums in-place. The order of the elements may be changed.
/// Then return the number of elements in nums which are not equal to val.
/// Consider the number of elements in nums which are not equal to val be k, to get accepted, you need to do the following things:
/// Change the array nums such that the first k elements of nums contain the elements which are not equal to val.
/// The remaining elements of nums are not important as well as the size of nums.
/// Return k.
pub fn remove_element_dummy(nums: &mut Vec<i32>, val: i32) -> i32 {
    use itertools::Itertools;
    use std::mem;
    let mut new = nums.iter().filter(|&x| x != &val).cloned().collect_vec();
    mem::swap(nums, &mut new);
    nums.len() as i32
}

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut j = nums.len() - 1;

    for i in 0..nums.len() {
        if i == j {
            break;
        }

        if nums[i] == val {
            for l in (i..=j).rev() {
                if nums[l] != val {
                    j = l;
                    break;
                }
            }

            nums.swap(i, j);
        }
    }

    j as i32
}

use itertools::Itertools;

#[test]
fn test_dummy() {
    cases().into_iter().for_each(|(mut nums, val, expected)| {
        let actual = remove_element_dummy(&mut nums, val) as usize;
        assert(nums, expected, actual);
    });
}

#[test]
fn test() {
    cases().into_iter().for_each(|(mut nums, val, expected)| {
        let actual = remove_element(&mut nums, val) as usize;
        assert(nums, expected, actual);
    });
}

fn assert(nums: Vec<i32>, expected: Vec<i32>, actual: usize) {
    let nums = nums.into_iter().take(expected.len()).sorted().collect_vec();
    assert_that!(nums).is_equal_to(&expected);
    assert_that!(actual).is_equal_to(expected.len());
}

fn cases() -> [(Vec<i32>, i32, Vec<i32>); 2] {
    [
        (vec![3, 2, 2, 3], 3, vec![2, 2]),
        (vec![0, 1, 2, 2, 3, 0, 4, 2], 2, vec![0, 0, 1, 3, 4]),
    ]
}
