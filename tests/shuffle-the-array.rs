//! https://leetcode.com/problems/shuffle-the-array
//!
//! Given the array nums consisting of 2n elements in the form [x1,x2,...,xn,y1,y2,...,yn].
//! Return the array in the form [x1,y1,x2,y2,...,xn,yn].

/// Pre-allocates with unsafe set_len and direct indexing.
///
/// Time: O(n) - single pass with direct assignment
/// Space: O(n) - allocates new vector of size 2n
///
/// Benchmarks (2n=total elements):
/// - small (2n=6): ~147 ns
/// - medium (2n=12): ~190 ns
/// - large (2n=100): ~524 ns
pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
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

/// what is probably expected
fn shuffle_2(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let n = n as usize;
    let mut ans = vec![0; n * 2];

    for i in 0..n {
        ans[i * 2] = nums[i];
        ans[i * 2 + 1] = nums[i + n];
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn test() {
        run_test(shuffle);
    }

    #[test]
    fn test_2() {
        run_test(shuffle_2);
    }

    fn run_test(target: fn(Vec<i32>, i32) -> Vec<i32>) {
        vec![
            (vec![2, 5, 1, 3, 4, 7], 3, vec![2, 3, 5, 4, 1, 7]),
            (vec![2, 5, 1, 3, 4, 7], 2, vec![2, 1, 5, 3]),
        ]
        .into_iter()
        .for_each(|(nums, n, expected)| {
            let actual = target(nums, n);
            let name = format!("{expected:?}");

            assert_that!(actual).named(&name).is_equal_to(&expected);
        });
    }
}
