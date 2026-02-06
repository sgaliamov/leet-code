//! <https://leetcode.com/problems/largest-rectangle-in-histogram/>
//!
//! Given an array of integers `heights` representing the histogram's bar height where the width
//! of each bar is 1, return the area of the largest rectangle in the histogram.
//!
//! # Example 1
//!
//! ```text
//! Input: heights = [2,1,5,6,2,3]
//! Output: 10
//! Explanation: The above is a histogram where width of each bar is 1.
//! The largest rectangle is shown in the red area, which has an area = 10 units.
//! ```
//!
//! # Example 2
//!
//! ```text
//! Input: heights = [2,4]
//! Output: 4
//! ```
//!
//! # Constraints
//!
//! - `1 <= heights.length <= 10⁵`
//! - `0 <= heights[i] <= 10⁴`

// Time Limit Exceeded
pub fn largest_rectangle_area_1(heights: Vec<i32>) -> i32 {
    let mut max = 0;

    for i in 0..heights.len() {
        let mut cnt = 0;

        for j in (0..i).rev() {
            if heights[i] < heights[j] {
                cnt += 1;
            } else {
                break;
            }
        }

        for j in i..heights.len() {
            if heights[i] <= heights[j] {
                cnt += 1;
            } else {
                break;
            }
        }

        max = max.max(cnt * heights[i]);
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        largest_rectangle_area_1,
    );

    fn run_test(target: fn(Vec<i32>) -> i32) {
        vec![
            (vec![2, 1, 5, 6, 2, 3], 10), // histogram with multiple bars
            (vec![2, 4], 4),              // two bars
        ]
        .into_iter()
        .for_each(|(heights, expected)| {
            let name = format!("heights={:?}", heights);
            let actual = target(heights);
            assert_that!(actual).named(&name).is_equal_to(expected);
        });
    }
}
