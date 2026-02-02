//! <https://leetcode.com/problems/exclusive-time-of-functions/>
//!
//! On a single-threaded CPU, we execute a program containing n functions. Each function has a
//! unique ID between 0 and n-1.
//!
//! Function calls are stored in a call stack: when a function call starts, its ID is pushed onto
//! the stack, and when a function call ends, its ID is popped off the stack. The function whose ID
//! is at the top of the stack is the current function being executed. Each time a function starts
//! or ends, we write a log with the ID, whether it started or ended, and the timestamp.
//!
//! You are given a list logs, where logs[i] represents the ith log message formatted as a string
//! "{function_id}:{"start" | "end"}:{timestamp}". For example, "0:start:3" means a function call
//! with function ID 0 started at the beginning of timestamp 3, and "1:end:2" means a function call
//! with function ID 1 ended at the end of timestamp 2. Note that a function can be called multiple
//! times, possibly recursively.
//!
//! A function's exclusive time is the sum of execution times for all function calls in the program.
//! For example, if a function is called twice, one call executing for 2 time units and another call
//! executing for 1 time unit, the exclusive time is 2 + 1 = 3.
//!
//! Return the exclusive time of each function in an array, where the value at the ith index
//! represents the exclusive time for the function with ID i.
//!
//! Constraints:
//! - 1 <= n <= 100
//! - 2 <= logs.length <= 500
//! - 0 <= function_id < n
//! - 0 <= timestamp <= 10^9
//! - No two start events will happen at the same timestamp
//! - No two end events will happen at the same timestamp
//! - Each function has an "end" log for each "start" log

// 2ms - 29.49% | 2.25MB - 93.59%
pub fn exclusive_time_1(n: i32, logs: Vec<String>) -> Vec<i32> {
    use itertools::Itertools;
    let mut counter = vec![0; n as usize];
    let mut stack = vec![];

    for rec in logs {
        let Some((id, act, time)) = rec.split(':').collect_tuple() else {
            break;
        };
        let id: usize = id.parse().unwrap();
        let time: i32 = time.parse().unwrap();

        if act == "start" {
            if let Some(&(id, start)) = stack.last() {
                let duration = time - start;
                counter[id] += duration;
            }

            stack.push((id, time));
        } else {
            let (id, start) = stack.pop().unwrap();
            let duration = time - start + 1;
            counter[id] += duration;

            if let Some((_, start)) = stack.last_mut() {
                *start = time + 1;
            }
        }
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        exclusive_time_1,
    );

    fn run_test(target: fn(i32, Vec<String>) -> Vec<i32>) {
        vec![
            (
                1,
                vec!["0:start:0".to_string(), "0:end:6".to_string()],
                vec![7],
            ),
            (
                2,
                vec![
                    "0:start:0".to_string(),
                    "1:start:2".to_string(),
                    "1:end:5".to_string(),
                    "0:end:6".to_string(),
                ],
                vec![3, 4],
            ),
            (
                1,
                vec![
                    "0:start:0".to_string(),
                    "0:start:2".to_string(),
                    "0:end:5".to_string(),
                    "0:start:6".to_string(),
                    "0:end:6".to_string(),
                    "0:end:7".to_string(),
                ],
                vec![8],
            ),
            (
                2,
                vec![
                    "0:start:0".to_string(),
                    "0:start:2".to_string(),
                    "0:end:5".to_string(),
                    "1:start:6".to_string(),
                    "1:end:6".to_string(),
                    "0:end:7".to_string(),
                ],
                vec![7, 1],
            ),
        ]
        .into_iter()
        .for_each(|(n, logs, expected)| {
            let name = format!("n={}, logs={:?}", n, logs);
            let actual = target(n, logs);
            assert_that!(actual).named(&name).is_equal_to(expected);
        });
    }
}
