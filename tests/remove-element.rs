//! https://leetcode.com/problems/remove-element

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

// from leet code
pub fn remove_element_perfect(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|&num| num != val);
    nums.len() as i32
}

// gemini
pub fn remove_element_swap_remove(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut i = 0;
    while i < nums.len() {
        if nums[i] == val {
            nums.swap_remove(i);
        } else {
            i += 1;
        }
    }
    nums.len() as i32
}

// my first ugly solution
pub fn remove_element_ugly(nums: &mut Vec<i32>, val: i32) -> i32 {
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

// the right solution, not mine.
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut i = 0;
    let mut j = 0;

    while i < nums.len() {
        if nums[i] != val {
            nums[j] = nums[i];
            j += 1;
        }

        i += 1;
    }

    j as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use spectral::prelude::*;

    #[test]
    fn test() {
        run_test(remove_element);
    }

    #[test]
    fn test_ugly() {
        run_test(remove_element_ugly);
    }

    #[test]
    fn test_dummy() {
        run_test(remove_element_dummy);
    }

    #[test]
    fn test_perfect() {
        run_test(remove_element_perfect);
    }

    #[test]
    fn test_swap_remove() {
        run_test(remove_element_swap_remove);
    }

    fn run_test(target: fn(&mut Vec<i32>, i32) -> i32) {
        vec![
            (vec![3, 2, 2, 3], 3, vec![2, 2]),
            (vec![2, 2, 3], 3, vec![2, 2]),
            (vec![2, 3], 3, vec![2]),
            (vec![3, 3], 3, vec![]),
            (vec![0, 1, 2, 2, 3, 0, 4, 2], 2, vec![0, 0, 1, 3, 4]),
            (
                vec![0, 1, 2, 2, 3, 0, 4, 2],
                5,
                vec![0, 0, 1, 2, 2, 2, 3, 4],
            ),
        ]
        .into_iter()
        .for_each(|(mut nums, val, expected)| {
            let actual = target(&mut nums, val) as usize;
            let nums = nums.into_iter().take(expected.len()).sorted().collect_vec();
            let name = format!("{nums:?} | {val} | {expected:?} | {actual}");

            assert_that!(nums).named(&name).is_equal_to(&expected);
            assert_that!(actual)
                .named(&name)
                .is_equal_to(expected.len());
        });
    }
}
