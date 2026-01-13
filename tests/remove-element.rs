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

// not mine
pub fn remove_element_perfect(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|&num| num != val);
    nums.len() as i32
}

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut j = nums.len();
    let mut i = 0;
    let mut s = 0;

    while i < j {
        if nums[i] == val {
            let mut found = false;
            s += 1;

            for l in (i + 1..j).rev() {
                if nums[l] != val {
                    found = true;
                    j = l;
                    break;
                } else {
                    j = l;
                    s += 1;
                }
            }

            if found {
                nums.swap(i, j);
            }
        }

        i += 1;
    }

    (nums.len() - s) as i32
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
fn test_perfect() {
    cases().into_iter().for_each(|(mut nums, val, expected)| {
        let actual = remove_element_perfect(&mut nums, val) as usize;
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

fn cases() -> Vec<(Vec<i32>, i32, Vec<i32>)> {
    vec![
        (vec![3, 2, 2, 3], 3, vec![2, 2]),
        (vec![2, 3], 3, vec![2]),
        (vec![3, 3], 3, vec![]),
        (vec![0, 1, 2, 2, 3, 0, 4, 2], 2, vec![0, 0, 1, 3, 4]),
        (
            vec![0, 1, 2, 2, 3, 0, 4, 2],
            5,
            vec![0, 0, 1, 2, 2, 2, 3, 4],
        ),
    ]
}

fn assert(nums: Vec<i32>, expected: Vec<i32>, actual: usize) {
    let nums = nums.into_iter().take(expected.len()).sorted().collect_vec();
    let name = format!("{nums:?} | {expected:?} | {actual}");

    assert_that!(nums).named(&name).is_equal_to(&expected);
    assert_that!(actual)
        .named(&name)
        .is_equal_to(expected.len());
}
