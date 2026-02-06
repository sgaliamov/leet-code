//! https://leetcode.com/problems/valid-sudoku/
//!
//! Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be validated
//! according to the following rules:
//!
//! 1. Each row must contain the digits 1-9 without repetition.
//! 2. Each column must contain the digits 1-9 without repetition.
//! 3. Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.
//!
//! Note:
//! - A Sudoku board (partially filled) could be valid but is not necessarily solvable.
//! - Only the filled cells need to be validated according to the mentioned rules.
//!
//! Constraints:
//! - `board.length == 9`
//! - `board[i].length == 9`
//! - `board[i][j]` is a digit 1-9 or '.'

// 0ms | 2.16MB - 93.10%
pub fn is_valid_sudoku_1(board: Vec<Vec<char>>) -> bool {
    use std::collections::HashSet;

    for r in &board {
        let mut row: HashSet<_> = HashSet::new();
        for &c in r {
            if c != '.' && !row.insert(c) {
                return false;
            }
        }
    }

    for c in 0..9 {
        let mut row: HashSet<_> = HashSet::new();
        for r in &board {
            if r[c] != '.' && !row.insert(r[c]) {
                return false;
            }
        }
    }

    for br in 0..3 {
        for bc in 0..3 {
            let mut row: HashSet<_> = HashSet::new();
            for r in 0..3 {
                for c in 0..3 {
                    let r = 3 * br + r;
                    let c = 3 * bc + c;

                    if board[r][c] != '.' && !row.insert(board[r][c]) {
                        return false;
                    }
                }
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        is_valid_sudoku_1,
    );

    fn run_test(target: fn(Vec<Vec<char>>) -> bool) {
        vec![
            (
                vec![
                    vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                    vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                    vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                    vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                    vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                    vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                    vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                    vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                    vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
                ],
                true,
            ),
            (
                vec![
                    vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                    vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                    vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                    vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                    vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                    vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                    vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                    vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                    vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
                ],
                false,
            ),
            (
                vec![
                    vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                    vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                    vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                    vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                    vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                    vec!['7', '4', '.', '.', '2', '.', '.', '.', '6'],
                    vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                    vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                    vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
                ],
                false,
            ),
            (
                vec![
                    vec!['.', '.', '.', '.', '5', '.', '.', '1', '.'],
                    vec!['.', '4', '.', '3', '.', '.', '.', '.', '.'],
                    vec!['.', '.', '.', '.', '.', '3', '.', '.', '1'],
                    vec!['8', '.', '.', '.', '.', '.', '.', '2', '.'],
                    vec!['.', '.', '2', '.', '7', '.', '.', '.', '.'],
                    vec!['.', '1', '5', '.', '.', '.', '.', '.', '.'],
                    vec!['.', '.', '.', '.', '.', '2', '.', '.', '.'],
                    vec!['.', '2', '.', '9', '.', '.', '.', '.', '.'],
                    vec!['.', '.', '4', '.', '.', '.', '.', '.', '.'],
                ],
                false,
            ),
            (
                vec![
                    vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                    vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                    vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                    vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                    vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                    vec!['7', '4', '.', '.', '2', '.', '.', '.', '6'],
                    vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                    vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                    vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
                ],
                false,
            ),
        ]
        .into_iter()
        .for_each(|(board, expected)| {
            let name = format!("board={:?}", board);
            let actual = target(board);
            assert_that!(actual).named(&name).is_equal_to(expected);
        });
    }
}
