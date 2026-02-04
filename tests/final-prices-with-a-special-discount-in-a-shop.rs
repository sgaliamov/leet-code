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
//! - 1 <= prices.length <= 500
//! - 1 <= prices[i] <= 1000

// 0ms | 2.07MB - 100% | O(n²)
pub fn final_prices_1(mut prices: Vec<i32>) -> Vec<i32> {
    for i in 0..prices.len() {
        let p = prices[i];
        let d = *prices.iter().skip(i + 1).find(|&x| x <= &p).unwrap_or(&0);
        prices[i] -= d;
    }

    prices
}

// 0ms | 2.19MB - 66.90%
pub fn final_prices_2(mut prices: Vec<i32>) -> Vec<i32> {
    use std::collections::VecDeque;

    fn push_back_decreasing<T: PartialOrd>(stack: &mut VecDeque<T>, val: T) {
        while let Some(last) = stack.back()
            && last < &val
        {
            stack.pop_back();
        }
        stack.push_back(val);
    }

    let mut stack = VecDeque::new();

    for i in 0..prices.len() - 1 {
        let price = prices[i];
        let next = prices[i + 1];

        if next <= price {
            prices[i] = price - next;
            stack.pop_front();
        } else {
            for j in i + 1..prices.len() {
                push_back_decreasing(&mut stack, prices[j]);
                if prices[j] <= price {
                    prices[i] = price - prices[j];
                    break;
                }
            }

            if stack.is_empty() {
                break;
            }
        }
    }

    prices
}

// 0ms | 2.07MB - 100%
pub fn final_prices_3(mut prices: Vec<i32>) -> Vec<i32> {
    fn push_back_increasing(stack: &mut Vec<i32>, val: i32) {
        while let Some(last) = stack.last() {
            if last < &val {
                break;
            } else if last == &val {
                return;
            }

            stack.pop();
        }

        stack.push(val);
    }

    let mut stack = Vec::new();

    for i in (0..prices.len()).rev() {
        let price = prices[i];

        while let Some(&top) = stack.last() {
            if top > price {
                stack.pop();
            } else {
                break;
            }
        }

        if let Some(top) = stack.last() {
            prices[i] -= top;
        }

        push_back_increasing(&mut stack, price);
    }

    prices
}

// canonical monotonic stack solution
pub fn final_prices_4(mut prices: Vec<i32>) -> Vec<i32> {
    let mut stack = Vec::new();

    for i in (0..prices.len()).rev() {
        let price = prices[i];

        while let Some(&top) = stack.last() {
            if top > price {
                stack.pop();
            } else {
                break;
            }
        }

        if let Some(top) = stack.last() {
            prices[i] -= top;
        }

        stack.push(price);
    }

    prices
}

pub fn final_prices_5(mut prices: Vec<i32>) -> Vec<i32> {
    prices
        .iter_mut()
        .rev()
        .fold(vec![], |mut stack, price_mut| {
            let price = *price_mut;

            while let Some(&top) = stack.last() {
                if top > price {
                    stack.pop();
                } else {
                    break;
                }
            }

            if let Some(top) = stack.last() {
                *price_mut -= top;
            }

            stack.push(price);
            stack
        });

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
        final_prices_2,
        final_prices_3,
        final_prices_4,
        final_prices_5
    );

    fn run_test(target: fn(Vec<i32>) -> Vec<i32>) {
        vec![
            (vec![4, 10, 6, 9, 8, 7, 5, 2], vec![2, 4, 1, 1, 1, 2, 3, 2]),
            (vec![4, 3, 2, 1], vec![1, 1, 1, 1]),
            (vec![8, 10, 9, 4, 6, 6, 2, 3], vec![4, 1, 5, 2, 0, 4, 2, 3]),
            (vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]),
            (vec![10, 1, 1, 6], vec![9, 0, 1, 6]),
        ]
        .into_iter()
        .for_each(|(prices, expected)| {
            let name = format!("prices = {:?}", prices);
            let actual = target(prices);
            assert_that!(actual).named(&name).is_equal_to(expected);
        });
    }
}
