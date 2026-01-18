//! https://leetcode.com/problems/set-mismatch
//!
//! You have a set of integers s, which originally contains all the numbers from 1 to n. Unfortunately, due to some error, one of the numbers in s got duplicated to another number in the set, which results in repetition of one number and loss of another number.
//! You are given an integer array nums representing the data status of this set after the error.
//! Find the number that occurs twice and the number that is missing and return them in the form of an array.

pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let mut d = 0;
    let mut set = std::collections::HashSet::<i32>::new();

    for &val in &nums {
        if !set.insert(val) {
            d = val;
        }
    }

    for val in 1..=nums.len() {
        let val = val as i32;
        if !set.contains(&val) {
            return vec![d, val];
        }
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn test() {
        run_test(find_error_nums);
    }

    fn run_test(target: fn(Vec<i32>) -> Vec<i32>) {
        vec![
            (vec![1, 5, 3, 2, 2, 7, 6, 4, 8, 9], vec![2, 10]),
            (vec![1, 2, 2, 4], vec![2, 3]),
            (vec![1, 1], vec![1, 2]),
            (vec![], vec![]),
            (vec![3, 2, 2], vec![2, 1]),
            (vec![2, 2], vec![2, 1]),
            (vec![1, 2, 3, 4], vec![]),
            (vec![3, 2, 3, 4, 6, 5], vec![3, 1]),
        ]
        .into_iter()
        .for_each(|(nums, expected)| {
            let name = format!("{nums:?}->{expected:?}");
            let actual = target(nums);

            assert_that!(actual).named(&name).is_equal_to(&expected);
        });
    }
}
