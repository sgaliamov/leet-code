//! https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number
//!
//! Given the array nums, for each nums[i] find out how many numbers in the array are smaller than it.
//! That is, for each nums[i] you have to count the number of valid j's such that j != i and nums[j] < nums[i].
//! Return the answer in an array.
//! Constraints:
//! - 2 <= nums.length <= 500
//! - 0 <= nums[i] <= 100

// 0ms | 70/2.16
pub fn smaller_numbers_than_current_1(nums: Vec<i32>) -> Vec<i32> {
    use std::collections::BTreeMap;

    let mut map = nums.iter().fold(BTreeMap::new(), |mut map, n| {
        map.entry(n).and_modify(|e| *e += 1).or_insert(1);
        map
    });

    let mut pv = 0;
    let mut c = 0;
    map.values_mut().for_each(|v| {
        c += pv;
        pv = *v;
        *v = c;
    });

    nums.iter()
        .enumerate()
        .fold(vec![0; nums.len()], |mut acc, (i, n)| {
            acc[i] = *map.get(n).unwrap();
            acc
        })
}

// 0ms | 30/2.24
pub fn smaller_numbers_than_current_2(nums: Vec<i32>) -> Vec<i32> {
    use itertools::Itertools;
    let mut sorted: Vec<_> = nums
        .iter()
        .enumerate()
        .sorted_unstable_by_key(|(_, n)| *n)
        .dedup_by_with_count(|a, b| a.1 == b.1)
        .map(|(c, (i, n))| (n, i, c))
        .collect();

    let mut pv = 0;
    let mut t = 0;
    sorted.iter_mut().for_each(|(_, _, c)| {
        t += pv;
        pv = *c;
        *c = t;
    });

    let mut res = sorted
        .iter()
        .fold(vec![-1; nums.len()], |mut acc, &(_, i, c)| {
            acc[i] = c as i32;
            acc
        });

    for i in 0..nums.len() {
        if res[i] == -1 {
            let n = nums[i];
            let j = sorted
                .binary_search_by(|&x| x.0.cmp(&n))
                .unwrap_or_else(|x| x);
            res[i] = sorted[j].2 as i32;
        }
    }

    res
}

// 0ms | 30/2.24
pub fn smaller_numbers_than_current_3(nums: Vec<i32>) -> Vec<i32> {
    let mut sorted = nums.clone();
    sorted.sort_unstable();
    nums.iter()
        .map(|&n| sorted.partition_point(|&x| x < n) as i32)
        .collect()
}

// 0ms | 30/2.24
pub fn smaller_numbers_than_current_4(nums: Vec<i32>) -> Vec<i32> {
    let mut sorted = nums.clone();
    sorted.sort_unstable();
    nums.iter()
        .fold(Vec::with_capacity(nums.len()), |mut acc, &n| {
            acc.push(sorted.partition_point(|&x| x < n) as i32);
            acc
        })
}

pub fn smaller_numbers_than_current_5(nums: Vec<i32>) -> Vec<i32> {
    let mut freq = vec![0_u16; 101];

    // Count frequency for each number (0 to 100)
    for &num in &nums {
        freq[num as usize] += 1;
    }

    // Build prefix sum: for each number, freq[i] becomes count of numbers ≤ i.
    for i in 1..101 {
        freq[i] += freq[i - 1];
    }

    // For each number, the answer is the count of numbers strictly less than it.
    nums.into_iter()
        .map(|num| {
            if num == 0 {
                0
            } else {
                freq[num as usize - 1] as i32
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        smaller_numbers_than_current_1,
        smaller_numbers_than_current_2,
        smaller_numbers_than_current_3,
        smaller_numbers_than_current_4,
        smaller_numbers_than_current_5
    );

    fn run_test(target: fn(Vec<i32>) -> Vec<i32>) {
        vec![
            (vec![6, 5, 4, 8], vec![2, 1, 0, 3]),
            (
                vec![
                    12, 23, 5, 7, 76, 2, 96, 27, 87, 4, 63, 90, 67, 92, 78, 56, 43, 84, 35, 15, 78,
                    88, 22, 12, 28, 51, 29, 63, 57, 48, 96, 21, 29, 30, 36, 59, 70, 74, 49, 27, 57,
                ],
                vec![
                    4, 9, 2, 3, 31, 0, 39, 10, 35, 1, 26, 37, 28, 38, 32, 22, 18, 34, 16, 6, 32,
                    36, 8, 4, 12, 21, 13, 26, 23, 19, 39, 7, 13, 15, 17, 25, 29, 30, 20, 10, 23,
                ],
            ),
            (vec![6, 3, 7, 6, 9], vec![1, 0, 3, 1, 4]),
            (vec![8, 1, 2, 2, 3], vec![4, 0, 1, 1, 3]),
            (vec![6, 5], vec![1, 0]),
            (vec![4, 5], vec![0, 1]),
            (vec![0, 0], vec![0, 0]),
            (vec![7, 7, 7, 7], vec![0, 0, 0, 0]),
            (vec![7, 0, 7], vec![1, 0, 1]),
        ]
        .into_iter()
        .for_each(|(nums, expected)| {
            let name = format!("{nums:?} {expected:?}");
            let actual = target(nums);

            assert_that!(actual).named(&name).is_equal_to(&expected);
        });
    }
}
