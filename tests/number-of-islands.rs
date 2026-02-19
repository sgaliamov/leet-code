//! https://leetcode.com/problems/number-of-islands/
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

// todo: solve
pub fn num_islands_1(mut grid: Vec<Vec<char>>) -> i32 {
    let mut cnt = b'1';

    let m = grid.len() - 1;
    for i in 0..=m {
        let n = grid[i].len() - 1;

        for j in 0..=n {
            if grid[i][j] == '0' || grid[i][j] != '1' {
                continue;
            }

            let lf = if j != 0 { grid[i][j - 1] } else { '0' };
            let rt = if j != n { grid[i][j + 1] } else { '0' };
            let up = if i != 0 { grid[i - 1][j] } else { '0' };
            let dw = if i != m { grid[i + 1][j] } else { '0' };

            let mut cur = up.max(dw).max(lf).max(rt).max(grid[i][j]);

            if cur == '1' {
                cnt += 1;
                cur = cnt as char;
            }

            grid[i][j] = cur;

            if up != '0' {
                grid[i - 1][j] = cur;
            }
            if dw != '0' {
                grid[i + 1][j] = cur;
            }
            if rt != '0' {
                grid[i][j + 1] = cur;
            }
        }
    }

    (cnt - b'1') as i32
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
                    vec!['1', '1', '0', '0', '0'],
                    vec!['1', '1', '0', '0', '0'],
                    vec!['0', '0', '1', '0', '0'],
                    vec!['0', '0', '0', '1', '1'],
                ],
                3,
            ),
            (
                vec![
                    vec!['1', '0', '1', '1', '1'],
                    vec!['1', '0', '1', '0', '1'],
                    vec!['1', '1', '1', '0', '1'],
                ],
                1,
            ),
            (
                vec![
                    vec!['1', '1', '1'],
                    vec!['0', '1', '0'],
                    vec!['1', '1', '1'],
                ],
                1,
            ),
            (
                vec![
                    vec!['1', '1', '1', '1', '0'],
                    vec!['1', '1', '0', '1', '0'],
                    vec!['1', '1', '0', '0', '0'],
                    vec!['0', '0', '0', '0', '0'],
                ],
                1,
            ),
            (
                vec![
                    vec!['0', '0', '0', '0', '0'],
                    vec!['0', '0', '0', '0', '0'],
                    vec!['0', '0', '0', '0', '0'],
                    vec!['0', '0', '0', '0', '0'],
                ],
                0,
            ),
        ]
        .into_iter()
        .enumerate()
        .for_each(|(i, (grid, expected))| {
            let name = format!("{}x{} grid {}", grid.len(), grid[0].len(), i + 1);
            let actual = target(grid);
            assert_that!(actual).named(&name).is_equal_to(expected);
        });
    }
}
