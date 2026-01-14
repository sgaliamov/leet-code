//! https://leetcode.com/problems/concatenation-of-array
//!
//! Given an integer array nums of length n, you want to create an array ans of length 2n where ans[i] == nums[i] and ans[i + n] == nums[i] for 0 <= i < n (0-indexed).
//! Specifically, ans is the concatenation of two nums arrays.
//! Return the array ans.

pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use spectral::prelude::*;

    #[test]
    fn test() {
        run_test(get_concatenation);
    }

    fn run_test(target: fn(Vec<i32>) -> Vec<i32>) {}
}
