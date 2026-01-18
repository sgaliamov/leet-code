//! https://leetcode.com/problems/set-mismatch
//!
//! You have a set of integers s, which originally contains all the numbers from 1 to n. Unfortunately, due to some error, one of the numbers in s got duplicated to another number in the set, which results in repetition of one number and loss of another number.
//! You are given an integer array nums representing the data status of this set after the error.
//! Find the number that occurs twice and the number that is missing and return them in the form of an array.

/// slow
pub fn find_error_nums_1(nums: Vec<i32>) -> Vec<i32> {
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

/// mine, passes, but takes O(n) space
pub fn find_error_nums_2(nums: Vec<i32>) -> Vec<i32> {
    let mut d = 0;
    let mut set = vec![false; nums.len() + 1];

    for &val in &nums {
        let i = val as usize;
        if set[i] {
            d = i;
        }
        set[i] = true;
    }

    #[allow(clippy::needless_range_loop)]
    for val in 1..=nums.len() {
        if !set[val] {
            return vec![d as i32, val as i32];
        }
    }

    vec![]
}

/// using sum, solved with a hint
pub fn find_error_nums_3(nums: Vec<i32>) -> Vec<i32> {
    let mut dup = 0_i32;
    let mut set = vec![false; nums.len()];
    let n = nums.len();
    let sum = (n * (1 + n) / 2) as i32;
    let mut actual = 0;

    for &val in &nums {
        let i = val as usize - 1;
        if set[i] {
            dup = val;
        }
        set[i] = true;
        actual += val;
    }

    let missing = (actual - sum - dup).abs();
    vec![dup, missing]
}

/// using XOR trick
/// XOR properties: a ^ a = 0, a ^ 0 = a, XOR is commutative and associative
/// If we XOR all numbers from 1 to n and all numbers in nums,
/// duplicates cancel out, leaving us with missing ^ duplicate
pub fn find_error_nums_4(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len() as i32;
    let mut xor = 0;

    // XOR all numbers from 1 to n and all numbers in nums
    for i in 1..=n {
        xor ^= i;
    }
    for &num in &nums {
        xor ^= num;
    }

    // xor now equals missing ^ duplicate
    // Find a set bit in xor (rightmost set bit)
    let rightmost_bit = xor & -xor;

    let mut x1 = 0;
    let mut x2 = 0;

    // Divide numbers into two groups based on the rightmost bit
    for i in 1..=n {
        if i & rightmost_bit != 0 {
            x1 ^= i;
        } else {
            x2 ^= i;
        }
    }

    for &num in &nums {
        if num & rightmost_bit != 0 {
            x1 ^= num;
        } else {
            x2 ^= num;
        }
    }

    // One of x1 or x2 is duplicate, the other is missing
    // Check which one appears in nums
    for &num in &nums {
        if num == x1 {
            return vec![x1, x2];
        }
    }

    vec![x2, x1]
}

/// using sum of squares formula
/// diff = expected_sum - actual_sum = missing - duplicate
/// diff2 = expected_sum_of_squares - actual_sum_of_squares = missing² - duplicate²
/// missing² - duplicate² = (missing - duplicate)(missing + duplicate)
/// So: missing + duplicate = diff2 / diff
/// Then: missing = (diff + missing + duplicate) / 2
pub fn find_error_nums_5(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len() as i64;
    let mut s = 0_i64;
    let mut s2 = 0_i64;

    for &num in &nums {
        let num = num as i64;
        s += num;
        s2 += num * num;
    }

    // Expected sum: n*(n+1)/2
    let diff = n * (n + 1) / 2 - s;
    // Expected sum of squares: n*(n+1)*(2n+1)/6
    let diff2 = n * (n + 1) * (2 * n + 1) / 6 - s2;

    let missing = (diff2 / diff + diff) / 2;
    let duplicate = missing - diff;

    vec![duplicate as i32, missing as i32]
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn test_1() {
        run_test(find_error_nums_1);
    }

    #[test]
    fn test_2() {
        run_test(find_error_nums_2);
    }

    #[test]
    fn test_3() {
        run_test(find_error_nums_3);
    }

    #[test]
    fn test_4() {
        run_test(find_error_nums_4);
    }

    #[test]
    fn test_5() {
        run_test(find_error_nums_5);
    }

    fn run_test(target: fn(Vec<i32>) -> Vec<i32>) {
        vec![
            (vec![1, 5, 3, 2, 2, 7, 6, 4, 8, 9], vec![2, 10]),
            (vec![1, 2, 2, 4], vec![2, 3]),
            (vec![1, 1], vec![1, 2]),
            (vec![3, 2, 2], vec![2, 1]),
            (vec![2, 2], vec![2, 1]),
            (vec![3, 2, 3, 4, 6, 5], vec![3, 1]),
        ]
        .into_iter()
        .for_each(|(nums, expected)| {
            let name = format!("{nums:?} → {expected:?}");
            let actual = target(nums);

            assert_that!(actual).named(&name).is_equal_to(&expected);
        });
    }
}
