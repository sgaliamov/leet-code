//! <https://leetcode.com/problems/number-of-islands/>
//!
//! Given an m x n 2D binary grid which represents a map of '1's (land) and '0's (water),
//! return the number of islands.
//!
//! An island is surrounded by water and is formed by connecting adjacent lands horizontally
//! or vertically. You may assume all four edges of the grid are all surrounded by water.
//!
//! Constraints:
//! - m == grid.length
//! - n == grid[i].length
//! - 1 <= m, n <= 300
//! - grid[i][j] is '0' or '1'

pub fn num_islands_1(grid: Vec<Vec<char>>) -> i32 {
    todo!("solve")
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        num_islands_1,
    );

    fn run_test(target: fn(Vec<Vec<char>>) -> i32) {
        vec![
            (
                vec![
                    vec!['1', '1', '1', '1', '0'],
                    vec!['1', '1', '0', '1', '0'],
                    vec!['1', '1', '0', '0', '0'],
                    vec!['0', '0', '0', '0', '0'],
                ],
                1,
            ), // Example 1: single large island
            (
                vec![
                    vec!['1', '1', '0', '0', '0'],
                    vec!['1', '1', '0', '0', '0'],
                    vec!['0', '0', '1', '0', '0'],
                    vec!['0', '0', '0', '1', '1'],
                ],
                3,
            ), // Example 2: three separate islands
        ]
        .into_iter()
        .for_each(|(grid, expected)| {
            let name = format!("{}x{} grid", grid.len(), grid[0].len());
            let actual = target(grid);
            assert_that!(actual).named(&name).is_equal_to(expected);
        });
    }
}
