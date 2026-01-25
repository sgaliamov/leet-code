//! https://leetcode.com/problems/summary-ranges
//!
//! You are given a sorted unique integer array nums.
//! A range [a,b] is the set of all integers from a to b (inclusive).
//! Return the smallest sorted list of ranges that cover all the numbers in the array exactly.
//! That is, each element of nums is covered by exactly one of the ranges, and there is no integer x such that x is in one of the ranges but not in nums.
//! Each range [a,b] in the list should be output as:
//! - "a->b" if a != b
//! - "a" if a == b

// 100/24/2.21
pub fn summary_ranges_1(nums: Vec<i32>) -> Vec<String> {
    if nums.is_empty() {
        return vec![];
    }

    if nums.len() == 1 {
        return vec![nums[0].to_string()];
    }

    let mut s = 0;
    let mut capacity = 1;
    for i in 1..nums.len() {
        if nums[i] != nums[i - 1] + 1 {
            capacity += 1;
        }
    }

    let mut ranges = Vec::with_capacity(capacity);

    for i in 1..nums.len() {
        if nums[i] != nums[i - 1] + 1 {
            let range = if s + 1 == i {
                format!("{}", nums[s])
            } else {
                format!("{}->{}", nums[s], nums[i - 1])
            };

            ranges.push(range);
            s = i;
        }
    }

    let range = if s == nums.len() - 1 {
        nums[s].to_string()
    } else {
        format!("{}->{}", nums[s], nums[nums.len() - 1])
    };

    ranges.push(range);
    ranges
}

// 100/86/2.19
pub fn summary_ranges_2(nums: Vec<i32>) -> Vec<String> {
    if nums.is_empty() {
        return vec![];
    }

    if nums.len() == 1 {
        return vec![nums[0].to_string(); 1];
    }

    let capacity = nums
        .iter()
        .enumerate()
        .skip(1)
        .filter(|(i, n)| **n != nums[i - 1] + 1)
        .count()
        + 1;

    let mut ranges = Vec::with_capacity(capacity);
    let mut s = 0;

    for i in 1..nums.len() {
        if nums[i] != nums[i - 1] + 1 {
            let range = if s + 1 == i {
                format!("{}", nums[s])
            } else {
                format!("{}->{}", nums[s], nums[i - 1])
            };

            ranges.push(range);
            s = i;
        }
    }

    let range = if s == nums.len() - 1 {
        nums[s].to_string()
    } else {
        format!("{}->{}", nums[s], nums[nums.len() - 1])
    };

    ranges.push(range);
    ranges
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    leet_code::solution_tests!(
        run_test:
        summary_ranges_1,
        summary_ranges_2,
    );

    fn run_test(target: fn(Vec<i32>) -> Vec<String>) {
        vec![
            (vec![], vec![]),
            (vec![0], vec!["0"]),
            (vec![0, 2, 3, 4, 6, 8, 9], vec!["0", "2->4", "6", "8->9"]),
            (vec![0, 1, 2, 4, 5, 7], vec!["0->2", "4->5", "7"]),
        ]
        .into_iter()
        .for_each(|(nums, expected)| {
            let name = format!("{nums:?} {expected:?}");
            let actual = target(nums);
            let actual: Vec<_> = actual.iter().map(|x| x.as_str()).collect();

            assert_that!(actual).named(&name).is_equal_to(&expected);
        });
    }
}
