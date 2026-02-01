//! https://leetcode.com/problems/max-consecutive-ones
//!
//! Given a binary array nums, return the maximum number of consecutive 1's in the array.
//!
//! Constraints:
//! - `1 <= nums.length <= 10^5`
//! - `nums[i]` is either `0` or `1`.

// 0ms | 2.68MB - 60.51%
pub fn find_max_consecutive_ones_1(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut c = 0;

    for i in 0..nums.len() {
        let x = nums[i];

        if x == 1 {
            c += 1;
        } else {
            if c > max {
                max = c;

                if nums.len() - i <= max {
                    break;
                }
            }
            c = 0;
        }
    }

    if c > max {
        max = c;
    }

    max as i32
}

// 0ms | 2.60MB - 60.51%
pub fn find_max_consecutive_ones_2(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut cnt = 0;

    for x in nums {
        if x == 1 {
            cnt += 1;
        } else if cnt > max {
            max = cnt;
            cnt = 0;
        } else {
            cnt = 0;
        }
    }

    cnt.max(max)
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        find_max_consecutive_ones_1,
        find_max_consecutive_ones_2,
    );

    fn run_test(target: fn(Vec<i32>) -> i32) {
        vec![
            (vec![1, 1, 0, 1, 1, 1], 3),
            (vec![1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1], 4),
        ]
        .into_iter()
        .for_each(|(nums, expected)| {
            let actual = target(nums);
            let name = format!("{expected:?}");

            assert_that!(actual).named(&name).is_equal_to(expected);
        });
    }
}
