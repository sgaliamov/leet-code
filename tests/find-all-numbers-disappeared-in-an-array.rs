//! https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array
//!
//! Given an array nums of n integers where nums[i] is in the range [1, n], return an array of all the integers in the range [1, n] that do not appear in nums.
//!
//! Example 1:
//! Input: nums = [4,3,2,7,8,2,3,1]
//! Output: [5,6]
//!
//! Example 2:
//! Input: nums = [1,1]
//! Output: [2]
//!
//! Constraints:
//! n == nums.length
//! 1 <= n <= 10^5
//! 1 <= nums[i] <= n
//! Follow up: Could you do it without extra space and in O(n) runtime? You may assume the returned list does not count as extra space.

/// U128 bitset approach - uses 782 u128 words to track presence.
///
/// Time: O(n) - two passes: mark present, collect missing
/// Space: O(1) - fixed 782 u128 array (12.5 KB) regardless of input
///
/// Benchmarks:
/// - small (n=8): 198 ns
/// - medium (n=1000): 3.94 µs
/// - large (n=10000): 34.5 µs
pub fn find_disappeared_numbers_1(nums: Vec<i32>) -> Vec<i32> {
    let mut used = vec![0_u128; 782];

    for n in &nums {
        let bucket = n / 128;
        let bit = 1 << (n - bucket * 128);
        used[bucket as usize] |= bit;
    }

    (1..=(nums.len() as i32))
        .filter(|n| {
            let bucket = n / 128;
            let bit = 1 << (n - bucket * 128);
            used[bucket as usize] & bit != bit
        })
        .collect()
}

/// Sort and deduplicate approach.
///
/// Time: O(n log n) - dominated by sorting
/// Space: O(log n) - sorting overhead
///
/// Benchmarks:
/// - small (n=8): 103 ns
/// - medium (n=1000): 1.73 µs
/// - large (n=10000): 136 µs
pub fn find_disappeared_numbers_2(nums: Vec<i32>) -> Vec<i32> {
    use itertools::Itertools;

    let mut res = Vec::new();
    let mut presented = nums.iter().sorted_unstable().dedup();
    let Some(mut v) = presented.next() else {
        return vec![];
    };

    for n in 1..=nums.len() as i32 {
        if &n != v {
            res.push(n);
        } else if let Some(new_v) = presented.next() {
            v = new_v;
        }
    }

    res
}

/// HashSet approach - simple but memory-intensive.
///
/// Time: O(n) - linear pass to build set, linear check
/// Space: O(n) - hashset stores all unique elements
///
/// Benchmarks:
/// - small (n=8): 263 ns
/// - medium (n=1000): 21.2 µs
/// - large (n=10000): 270 µs
pub fn find_disappeared_numbers_3(nums: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();
    let set: std::collections::HashSet<_> = nums.iter().collect();

    for n in 1..=nums.len() as i32 {
        if !set.contains(&n) {
            res.push(n);
        }
    }

    res
}

/// In-place negation approach - marks presence by negating values.
///
/// Time: O(n) - two linear passes
/// Space: O(1) - modifies input array, no extra allocation
///
/// Benchmarks:
/// - small (n=8): 60 ns
/// - medium (n=1000): 1.18 µs
/// - large (n=10000): 18.6 µs ⚡ fastest at scale
pub fn find_disappeared_numbers_4(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let mut i = 0;

    while i < nums.len() {
        let j = nums[i].unsigned_abs() as usize - 1;

        if nums[j] > 0 {
            nums[j] = -nums[j];
        }
        i += 1;
    }
    i = 0;
    let mut j = 0;

    while i < nums.len() {
        if nums[i] > 0 {
            nums[j] = i as i32 + 1;
            j += 1;
        }
        i += 1;
    }

    nums.truncate(j);
    nums
}

/// Optimized u64 bitset with unsafe operations and bit manipulation.
///
/// Time: O(n) - linear marking + efficient bit scanning with trailing_zeros
/// Space: O(n/64) - dynamically sized bitset, ~157 bytes for n=10000
///
/// Benchmarks:
/// - small (n=8): 59 ns ⚡ fastest overall
/// - medium (n=1000): 868 ns ⚡ fastest
/// - large (n=10000): 20.2 µs
pub fn find_disappeared_numbers_5(mut nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let num_words = len.div_ceil(64);
    let mut mask = vec![0u64; num_words];

    for &x in &nums {
        let val = x as usize - 1;
        unsafe {
            *mask.get_unchecked_mut(val / 64) |= 1 << (val % 64);
        }
    }

    let mut count = 0;
    for i in 0..num_words {
        let word = unsafe { *mask.get_unchecked(i) };
        if word == u64::MAX {
            continue;
        }

        let mut not_word = !word;
        while not_word != 0 {
            let trailing = not_word.trailing_zeros();
            let val = (i * 64) + trailing as usize + 1;

            if val > len {
                break;
            }

            unsafe {
                *nums.get_unchecked_mut(count) = val as i32;
            }
            count += 1;

            // Clear the lowest set bit
            not_word &= not_word - 1;
        }
    }

    nums.truncate(count);
    nums
}

pub fn find_disappeared_numbers_6(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let num_words = len.div_ceil(64);
    let mut used = vec![0u64; num_words];

    for n in &nums {
        let bucket = n / 64;
        let bit = 1 << (n - bucket * 64);
        used[bucket as usize] |= bit;
    }

    (1..=(nums.len() as i32))
        .filter(|n| {
            let bucket = n / 64;
            let bit = 1 << (n - bucket * 64);
            used[bucket as usize] & bit != bit
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        find_disappeared_numbers_1,
        find_disappeared_numbers_2,
        find_disappeared_numbers_3,
        find_disappeared_numbers_4,
        find_disappeared_numbers_5,
        find_disappeared_numbers_6,
    );

    fn run_test(target: fn(Vec<i32>) -> Vec<i32>) {
        vec![
            (vec![4, 3, 2, 7, 8, 2, 3, 1], vec![5, 6]),
            (
                vec![
                    98, 200, 14, 106, 113, 91, 6, 6, 192, 155, 9, 70, 189, 108, 5, 14, 178, 79,
                    172, 99, 136, 5, 168, 139, 44, 178, 28, 107, 75, 171, 93, 64, 82, 61, 31, 125,
                    29, 12, 116, 38, 153, 124, 63, 167, 72, 95, 141, 156, 21, 183, 94, 38, 97, 10,
                    111, 196, 158, 35, 42, 106, 197, 195, 98, 94, 96, 140, 53, 31, 73, 138, 172,
                    137, 104, 66, 26, 101, 93, 162, 151, 60, 18, 87, 136, 52, 36, 62, 20, 60, 54,
                    194, 149, 65, 122, 100, 17, 35, 69, 9, 53, 194, 158, 77, 154, 181, 34, 197,
                    119, 114, 192, 183, 164, 41, 3, 69, 193, 190, 71, 184, 11, 143, 29, 15, 162,
                    176, 1, 32, 50, 95, 184, 68, 112, 144, 21, 91, 11, 28, 141, 49, 121, 148, 12,
                    188, 144, 82, 52, 152, 87, 16, 17, 15, 37, 186, 68, 122, 57, 111, 71, 160, 169,
                    13, 116, 66, 130, 61, 131, 123, 128, 48, 103, 199, 48, 102, 200, 23, 70, 195,
                    25, 30, 72, 142, 176, 46, 7, 180, 20, 112, 99, 42, 179, 151, 33, 190, 180, 199,
                    174, 104, 22, 49, 191, 157,
                ],
                vec![
                    2, 4, 8, 19, 24, 27, 39, 40, 43, 45, 47, 51, 55, 56, 58, 59, 67, 74, 76, 78,
                    80, 81, 83, 84, 85, 86, 88, 89, 90, 92, 105, 109, 110, 115, 117, 118, 120, 126,
                    127, 129, 132, 133, 134, 135, 145, 146, 147, 150, 159, 161, 163, 165, 166, 170,
                    173, 175, 177, 182, 185, 187, 198,
                ],
            ),
            (vec![1, 2, 3], vec![]),
            (vec![1, 1], vec![2]),
        ]
        .into_iter()
        .for_each(|(nums, expected)| {
            let name = format!("{nums:?} {expected:?}");
            let actual = target(nums).into_iter().sorted_unstable().collect_vec();

            assert_that!(actual).named(&name).is_equal_to(&expected);
        });
    }
}
