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
    let mut marker = 2_u8;
    let mut cnt = 0;
    let mut map = std::collections::HashMap::new();
    let mut active = false;

    let m = grid.len() - 1;
    for i in 0..=m {
        let n = grid[i].len() - 1;

        for j in 0..=n {
            if grid[i][j] == '0' {
                if active {
                    marker += 1;
                    active = false;
                }
                continue;
            } else {
                active = true;
            }

            let up = if i != 0 { grid[i - 1][j] } else { '0' };
            if up == '0' {
                grid[i][j] = marker as char;
                map.entry(grid[i][j]).or_insert_with(|| {
                    cnt += 1;
                    '0'
                });
            } else {
                let c = marker as char;
                match map.get_mut(&c) {
                    Some(known) => {
                        if known != &up {
                            *known = up;
                            cnt -= 1;
                        }
                    }
                    None => {
                        map.entry(c).or_insert_with(|| {
                            // cnt -= 1;
                            up
                        });
                    }
                }
            }
        }

        active = false;
        marker += 1;
    }

    cnt
}

pub fn num_islands_2(mut grid: Vec<Vec<char>>) -> i32 {
    fn explore(i: usize, j: usize, grid: &mut Vec<Vec<char>>) {
        let m = grid.len() - 1;
        for i in i..=m {
            let n = grid[i].len() - 1;

            for j in 0..=n {
                let c = grid[i][j] as u8 - b'1';

                let lf = if j != 0 { grid[i][j - 1] } else { '0' };
                let rt = if j != n { grid[i][j + 1] } else { '0' };
                let up = if i != 0 { grid[i - 1][j] } else { '0' };
                let dw = if i != m { grid[i + 1][j] } else { '0' };

                if c == 0 {
                    continue;
                }
            }
        }
    }

    let mut cnt = b'1';

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let c = grid[i][j] as u8 - b'1';
            if c != 1 {
                continue;
            }

            explore(i, j, &mut grid);
        }
    }

    0
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
                    vec!['1', '0', '1'],
                    vec!['1', '0', '1'],
                    vec!['1', '0', '1'],
                ],
                2,
            ),
            (
                vec![
                    vec!['1', '0', '1'], //
                    vec!['1', '0', '1'],
                ],
                2,
            ),
            (
                vec![
                    vec!['1', '1', '0', '1', '1'], //
                    vec!['0', '1', '1', '1', '1'],
                ],
                1,
            ),
            (
                vec![
                    vec!['0', '0', '1', '1'], //
                    vec!['1', '1', '1', '0'],
                ],
                1,
            ),
            (
                vec![
                    vec!['1', '1'], //
                    vec!['1', '1'],
                ],
                1,
            ),
            (vec![vec!['1']], 1),
            (
                vec![
                    vec!['1', '1', '1', '1', '1'],
                    vec!['1', '0', '1', '0', '1'],
                    vec!['1', '0', '1', '0', '1'],
                ],
                1,
            ),
            (
                vec![
                    vec!['1', '0', '1', '0', '1'],
                    vec!['1', '0', '1', '0', '1'],
                    vec!['1', '1', '1', '1', '1'],
                ],
                1,
            ),
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
