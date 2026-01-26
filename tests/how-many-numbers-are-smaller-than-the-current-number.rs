//! https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number
//!
//! Given the array nums, for each nums[i] find out how many numbers in the array are smaller than it.
//! That is, for each nums[i] you have to count the number of valid j's such that j != i and nums[j] < nums[i].
//! Return the answer in an array.

pub fn smaller_numbers_than_current_1(nums: Vec<i32>) -> Vec<i32> {
    use std::collections::BTreeMap;

    let mut map = BTreeMap::new();
    (0..nums.len()).for_each(|i| {
        map.entry(nums[i]).and_modify(|e| *e += 1).or_insert(1);
    });

    let mut pv = 0;
    let mut c = 0;

    map.iter_mut().for_each(|(&_, v)| {
        c += pv;
        pv = *v;
        *v = c;
    });

    let mut res = vec![0; nums.len()];

    (0..nums.len()).for_each(|i| {
        let n = nums[i];
        res[i] = *map.get(&n).unwrap();
    });

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        smaller_numbers_than_current_1,
    );

    fn run_test(target: fn(Vec<i32>) -> Vec<i32>) {
        vec![
            (vec![8, 1, 2, 2, 3], vec![4, 0, 1, 1, 3]),
            (vec![6, 5, 4, 8], vec![2, 1, 0, 3]),
            (vec![6, 5], vec![1, 0]),
            (vec![4, 5], vec![0, 1]),
            (vec![0, 0], vec![0, 0]),
            (vec![7, 7, 7, 7], vec![0, 0, 0, 0]),
        ]
        .into_iter()
        .for_each(|(nums, expected)| {
            let name = format!("{nums:?} {expected:?}");
            let actual = target(nums);

            assert_that!(actual).named(&name).is_equal_to(&expected);
        });
    }
}
