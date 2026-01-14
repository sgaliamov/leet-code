//! https://leetcode.com/problems/shuffle-the-array
//!
//! Given the array nums consisting of 2n elements in the form [x1,x2,...,xn,y1,y2,...,yn].
//! Return the array in the form [x1,y1,x2,y2,...,xn,yn].

pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
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
