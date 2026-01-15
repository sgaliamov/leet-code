//! https://leetcode.com/problems/max-consecutive-ones
//!
//! Given a binary array nums, return the maximum number of consecutive 1's in the array.

pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn test() {
        run_test(find_max_consecutive_ones);
    }

    fn run_test(target: fn(Vec<i32>) -> i32) {
        vec![(vec![1,1,0,1,1,1], 3)]
            .into_iter()
            .for_each(|(nums, expected)| {
                let actual = target(nums);
                let name = format!("{expected:?}");

                assert_that!(actual).named(&name).is_equal_to(expected);
            });
    }
}
