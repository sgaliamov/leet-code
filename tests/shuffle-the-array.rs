//! https://leetcode.com/problems/shuffle-the-array
//!
//! Given the array nums consisting of 2n elements in the form [x1,x2,...,xn,y1,y2,...,yn].
//! Return the array in the form [x1,y1,x2,y2,...,xn,yn].
//!
//! Constraints:
//! 1 <= n <= 500
//! nums.length == 2n
//! 1 <= nums[i] <= 10^3

// 1ms - 79.38% | 2.26MB - 30.11%
pub fn shuffle_1(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let n = n as usize;
    let mut ans = Vec::with_capacity(n * 2);

    #[allow(clippy::uninit_vec)]
    unsafe {
        ans.set_len(n * 2);
    }

    for i in 0..n {
        ans[i * 2] = nums[i];
        ans[i * 2 + 1] = nums[i + n];
    }

    ans
}

// 1ms - 79.38% | 2.21MB - 30.11%
fn shuffle_2(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let n = n as usize;
    let mut ans = vec![0; n * 2];

    for i in 0..n {
        ans[i * 2] = nums[i];
        ans[i * 2 + 1] = nums[i + n];
    }

    ans
}

// 0ms | 2.26MB - 30.11%
fn shuffle_3(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let mut nums = nums;
    let n = n as usize;

    for i in 0..n {
        nums[i] = (nums[i] * 10000) + nums[n + i];
    }

    for i in (0..n).rev() {
        nums[2 * i + 1] = nums[i] % 10000;
        nums[2 * i] = nums[i] / 10000;
    }

    nums.truncate(n * 2);
    nums
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        shuffle_1,
        shuffle_2,
        shuffle_3,
    );

    fn run_test(target: fn(Vec<i32>, i32) -> Vec<i32>) {
        vec![
            (vec![2, 5, 1, 3, 4, 7], 3, vec![2, 3, 5, 4, 1, 7]),
            (vec![2, 5, 1, 3, 4, 7], 2, vec![2, 1, 5, 3]),
        ]
        .into_iter()
        .for_each(|(nums, n, expected)| {
            let name = format!("{nums:?} {expected:?}");
            let actual = target(nums, n);

            assert_that!(actual).named(&name).is_equal_to(&expected);
        });
    }
}
