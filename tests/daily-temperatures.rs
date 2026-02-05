//! https://leetcode.com/problems/daily-temperatures/
//!
//! Given an array of integers `temperatures` represents the daily temperatures,
//! return an array `answer` such that `answer[i]` is the number of days you have
//! to wait after the ith day to get a warmer temperature. If there is no future
//! day for which this is possible, keep `answer[i] == 0` instead.
//!
//! Constraints:
//! - 1 <= temperatures.length <= 10^5
//! - 30 <= temperatures[i] <= 100

// 9ms - 21.86% | 3.75MB - 86.50%
pub fn daily_temperatures_1(mut temperatures: Vec<i32>) -> Vec<i32> {
    let mut stack = Vec::<(usize, i32)>::new();

    for t in temperatures.iter_mut().enumerate().rev() {
        while stack.last().is_some_and(|(_, v)| v <= t.1) {
            stack.pop();
        }

        let v = *t.1;
        if let Some((i, _)) = stack.last() {
            *t.1 = (i - t.0) as i32;
        } else {
            *t.1 = 0;
        }

        stack.push((t.0, v));
    }

    temperatures
}

// 4ms - 66.65% | 3.82MB - 84.24%
pub fn daily_temperatures_2(mut temperatures: Vec<i32>) -> Vec<i32> {
    let mut stack = Vec::<(usize, i32)>::new();
    let mut j = temperatures.len();

    for t in temperatures.iter_mut().rev() {
        j -= 1;

        while stack.last().is_some_and(|(_, v)| v <= t) {
            stack.pop();
        }

        let v = *t;
        if let Some((i, _)) = stack.last() {
            *t = (i - j) as i32;
        } else {
            *t = 0;
        }

        stack.push((j, v));
    }

    temperatures
}

// 0ms | 3.81MB - 84.24%
pub fn daily_temperatures_3(mut temperatures: Vec<i32>) -> Vec<i32> {
    let mut stack = Vec::<(usize, i32)>::new();

    for j in (0..temperatures.len()).rev() {
        let t = temperatures[j];

        // do not have to use tuple - can keep index only in the stack
        while stack.last().is_some_and(|&(_, v)| v <= t) {
            stack.pop();
        }

        temperatures[j] = stack.last().map_or(0, |(i, _)| (i - j) as i32);
        stack.push((j, t));
    }

    temperatures
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        daily_temperatures_1,
        daily_temperatures_2,
        daily_temperatures_3,
    );

    fn run_test(target: fn(Vec<i32>) -> Vec<i32>) {
        vec![
            (
                vec![89, 62, 70, 58, 47, 47, 46, 76, 100, 70],
                vec![8, 1, 5, 4, 3, 2, 1, 1, 0, 0],
            ),
            (
                vec![73, 74, 75, 71, 69, 72, 76, 73],
                vec![1, 1, 4, 2, 1, 1, 0, 0],
            ),
            (vec![30, 40, 50, 60], vec![1, 1, 1, 0]),
            (vec![30, 60, 90], vec![1, 1, 0]),
        ]
        .into_iter()
        .for_each(|(temperatures, expected)| {
            let name = format!("daily_temperatures({:?})", temperatures);
            let actual = target(temperatures);
            assert_that!(actual).named(&name).is_equal_to(expected);
        });
    }
}
