//! https://leetcode.com/problems/final-prices-with-a-special-discount-in-a-shop/
//!
//! You are given an integer array `prices` where `prices[i]` is the price of the ith item in a shop.
//!
//! There is a special discount for items in the shop. If you buy the ith item, then you will receive
//! a discount equivalent to `prices[j]` where `j` is the minimum index such that `j > i` and
//! `prices[j] <= prices[i]`. Otherwise, you will not receive any discount at all.
//!
//! Return an integer array `answer` where `answer[i]` is the final price you will pay for the ith item
//! of the shop, considering the special discount.
//!
//! Constraints:
//! - `1 <= prices.length <= 500`
//! - `1 <= prices[i] <= 1000`

use leet_code::push_decreasing;

// 0ms | 2.07MB - 100% | O(n²)
pub fn final_prices_1(mut prices: Vec<i32>) -> Vec<i32> {
    for i in 0..prices.len() {
        let p = prices[i];
        let d = *prices.iter().skip(i + 1).find(|&x| x <= &p).unwrap_or(&0);
        prices[i] -= d;
    }

    prices
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        final_prices_1,
    );

    fn run_test(target: fn(Vec<i32>) -> Vec<i32>) {
        vec![
            (vec![8, 4, 6, 2, 3], vec![4, 2, 4, 2, 3]), // Example 1: discounts applied
            (vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]), // Example 2: no discounts
            (vec![10, 1, 1, 6], vec![9, 0, 1, 6]),      // Example 3: mixed discounts
        ]
        .into_iter()
        .for_each(|(prices, expected)| {
            let name = format!("prices = {:?}", prices);
            let actual = target(prices);
            assert_that!(actual).named(&name).is_equal_to(expected);
        });
    }
}
