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
// Time Limit Exceeded
pub fn num_islands_0(mut grid: Vec<Vec<char>>) -> i32 {
    let mut marker = 2_u8;
    let mut map = std::collections::HashMap::new();
    let mut active = false;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '0' {
                if active {
                    marker += 1;
                    active = false;
                }
                continue;
            } else {
                active = true;
            }

            let c = marker as char;
            let up = if i != 0 { grid[i - 1][j] } else { '0' };
            grid[i][j] = c;

            if up == '0' {
                map.entry(c).or_insert(c);
            } else {
                let root = *map.get(&up).unwrap();

                let Some(&known) = map.get(&c) else {
                    map.entry(c).or_insert(root);
                    continue;
                };

                if known != root {
                    map.entry(root).and_modify(|e| *e = known);
                }
            }
        }

        active = false;
        marker += 1;
    }

    let unique: std::collections::HashSet<_> = map
        .values()
        .map(|mut c| {
            while let Some(root) = map.get(c) {
                if c == root {
                    return root;
                }
                c = root;
            }
            c
        })
        .collect();
    unique.len() as i32
}

// Time Limit Exceeded
pub fn num_islands_1(mut grid: Vec<Vec<char>>) -> i32 {
    fn explore(i: u32, j: u32, grid: &mut [Vec<char>], mark: char) {
        let mut stack = vec![(i << 10) | j];
        let m = grid.len() - 1;

        while let Some(v) = stack.pop() {
            let i = (v >> 10) as usize;
            let j = (v & 0b1111111111) as usize;
            grid[i][j] = mark;
            let n = grid[i].len() - 1;

            let up = if i != 0 { grid[i - 1][j] } else { '0' };
            let dw = if i != m { grid[i + 1][j] } else { '0' };
            let lf = if j != 0 { grid[i][j - 1] } else { '0' };
            let rt = if j != n { grid[i][j + 1] } else { '0' };

            if dw == '1' {
                let v = ((i + 1) << 10) | j;
                stack.push(v as u32);
            }

            if lf == '1' {
                let v = (i << 10) | (j - 1);
                stack.push(v as u32);
            }

            if up == '1' {
                let v = ((i - 1) << 10) | j;
                stack.push(v as u32);
            }

            if rt == '1' {
                let v = (i << 10) | (j + 1);
                stack.push(v as u32);
            }
        }
    }

    let mut cnt = b'2';

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let c = grid[i][j];
            if c != '1' {
                continue;
            }

            explore(i as u32, j as u32, &mut grid, cnt as char);
            cnt += 1;
        }
    }

    (cnt - b'2') as i32
}

// Time Limit Exceeded
pub fn num_islands_2(mut grid: Vec<Vec<char>>) -> i32 {
    fn explore(i: u16, j: u16, grid: &mut [Vec<char>], mark: char) {
        let mut stack = vec![(i, j)];
        let m = grid.len() - 1;

        while let Some((i, j)) = stack.pop() {
            let i = i as usize;
            let j = j as usize;
            grid[i][j] = mark;
            let n = grid[i].len() - 1;

            let up = if i != 0 { grid[i - 1][j] } else { '0' };
            let dw = if i != m { grid[i + 1][j] } else { '0' };
            let lf = if j != 0 { grid[i][j - 1] } else { '0' };
            let rt = if j != n { grid[i][j + 1] } else { '0' };

            let i = i as u16;
            let j = j as u16;

            if dw == '1' {
                stack.push((i + 1, j));
            }

            if lf == '1' {
                stack.push((i, j - 1));
            }

            if up == '1' {
                stack.push((i - 1, j));
            }

            if rt == '1' {
                stack.push((i, j + 1));
            }
        }
    }

    let mut cnt = b'2';

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let c = grid[i][j];
            if c != '1' {
                continue;
            }

            explore(i as u16, j as u16, &mut grid, cnt as char);
            cnt += 1;
        }
    }

    (cnt - b'2') as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        num_islands_0,
        num_islands_1,
        num_islands_2,
    );

    fn run_test(target: fn(Vec<Vec<char>>) -> i32) {
        vec![
            (
                vec![
                    vec!['1', '0', '1', '1', '1'],
                    vec!['1', '0', '1', '0', '1'],
                    vec!['1', '0', '1', '0', '1'],
                    vec!['1', '1', '1', '0', '1'],
                ],
                1,
            ),
            (
                vec![
                    vec!['1', '1', '1', '0', '1'],
                    vec!['1', '0', '1', '0', '1'],
                    vec!['1', '0', '1', '0', '1'],
                    vec!['1', '0', '1', '1', '1'],
                ],
                1,
            ),
            (
                vec![
                    vec!['1', '1', '1', '1', '1'],
                    vec!['1', '0', '1', '0', '1'],
                    vec!['1', '0', '1', '0', '1'],
                    vec!['1', '0', '1', '0', '1'],
                ],
                1,
            ),
            (
                vec![
                    vec!['1', '0', '1', '0', '1'],
                    vec!['1', '0', '1', '0', '1'],
                    vec!['1', '0', '1', '0', '1'],
                    vec!['1', '1', '1', '1', '1'],
                ],
                1,
            ),
            (
                vec![
                    vec!['1', '1', '1', '1', '1', '0', '1', '1', '1', '1'],
                    vec!['1', '0', '1', '0', '1', '1', '1', '1', '1', '1'],
                    vec!['0', '1', '1', '1', '0', '1', '1', '1', '1', '1'],
                    vec!['1', '1', '0', '1', '1', '0', '0', '0', '0', '1'],
                    vec!['1', '0', '1', '0', '1', '0', '0', '1', '0', '1'],
                    vec!['1', '0', '0', '1', '1', '1', '0', '1', '0', '0'],
                    vec!['0', '0', '1', '0', '0', '1', '1', '1', '1', '0'],
                    vec!['1', '0', '1', '1', '1', '0', '0', '1', '1', '1'],
                    vec!['1', '1', '1', '1', '1', '1', '1', '1', '0', '1'],
                    vec!['1', '0', '1', '1', '1', '1', '1', '1', '1', '0'],
                ],
                2,
            ),
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
                    vec!['1', '1', '0', '0', '0'],
                    vec!['1', '1', '0', '0', '0'],
                    vec!['0', '0', '1', '0', '0'],
                    vec!['0', '0', '0', '1', '1'],
                ],
                3,
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
