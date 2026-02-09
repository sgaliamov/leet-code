//! https://leetcode.com/problems/average-of-levels-in-binary-tree/
//!
//! Given the root of a binary tree, return the average value of the nodes on each level
//! in the form of an array. Answers within 10^-5 of the actual answer will be accepted.
//!
//! Example 1:
//! Input: root = [3,9,20,null,null,15,7]
//! Output: [3.00000,14.50000,11.00000]
//! Explanation: The average value of nodes on level 0 is 3, on level 1 is 14.5, and on level 2 is 11.
//!
//! Example 2:
//! Input: root = [3,9,20,15,7]
//! Output: [3.00000,14.50000,11.00000]
//!
//! Constraints:
//! - The number of nodes in the tree is in the range [1, 10^4]
//! - -2^31 <= Node.val <= 2^31 - 1

use std::cell::RefCell;
use std::rc::Rc;

// 2ms - 3.85% | 3.02MB - 57.69%
pub fn average_of_levels_1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
    use std::collections::VecDeque;

    let mut queue = VecDeque::new();
    let mut averages = vec![0.0];
    let mut cur = 0;
    let mut cnt = 0;
    queue.push_back((0, root.unwrap().clone()));

    while let Some((level, node)) = queue.pop_front() {
        if let Some(node) = &node.borrow().left {
            queue.push_back((level + 1, node.clone()));
        }

        if let Some(node) = &node.borrow().right {
            queue.push_back((level + 1, node.clone()));
        }

        let val = node.borrow().val as f64;

        if level == cur {
            averages[cur] += val;
            cnt += 1;
        } else {
            averages[cur] /= cnt as f64;
            cur = level;
            averages.push(val);
            cnt = 1;
        }
    }

    averages[cur] /= cnt as f64;
    averages
}

// 0ms | 3.07MB - 57.69%
pub fn average_of_levels_2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
    use std::collections::VecDeque;

    let mut queue = VecDeque::new();
    let mut averages = vec![];
    let mut cur = 0;
    let mut sum = 0_i64;
    let mut cnt = 0;
    queue.push_back((0, root.unwrap().clone()));

    while let Some((level, node)) = queue.pop_front() {
        if let Some(node) = &node.borrow().left {
            queue.push_back((level + 1, node.clone()));
        }

        if let Some(node) = &node.borrow().right {
            queue.push_back((level + 1, node.clone()));
        }

        let val = node.borrow().val as i64;

        if level == cur {
            sum += val;
            cnt += 1;
        } else {
            averages.push(sum as f64 / cnt as f64);
            cur = level;
            sum = val;
            cnt = 1;
        }
    }

    averages.push(sum as f64 / cnt as f64);
    averages
}

/// Claude 4.5
/// Level-order BFS with two-queue approach for clean level separation.
/// Processes all nodes at current level before moving to next level.
///
/// Time: O(n) - visit each node exactly once
/// Space: O(w) - where w is maximum width of tree (worst case n/2 for perfect tree)
pub fn average_of_levels_3(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
    use std::collections::VecDeque;

    let mut result = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(root.unwrap());

    while !queue.is_empty() {
        let mut sum = 0i64;
        let count = queue.len();

        for _ in 0..count {
            let node = queue.pop_front().unwrap();
            let borrowed = node.borrow();
            sum += borrowed.val as i64;

            if let Some(left) = &borrowed.left {
                queue.push_back(left.clone());
            }
            if let Some(right) = &borrowed.right {
                queue.push_back(right.clone());
            }
        }

        result.push(sum as f64 / count as f64);
    }

    result
}

use leet_code::tree::{TreeNode, build_tree};

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        average_of_levels_1,
        average_of_levels_2,
        average_of_levels_3,
    );

    fn run_test(target: fn(Option<Rc<RefCell<TreeNode>>>) -> Vec<f64>) {
        vec![
            (
                vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
                vec![3.00000, 14.50000, 11.00000],
            ),
            (
                vec![Some(3), Some(9), Some(20), Some(15), Some(7)],
                vec![3.00000, 14.50000, 11.00000],
            ),
            (
                vec![Some(2147483647), Some(2147483647), Some(2147483647)],
                vec![2147483647.0, 2147483647.0],
            ),
        ]
        .into_iter()
        .for_each(|(tree, expected)| {
            let root = build_tree(&tree);
            let name = format!("tree={:?}", tree);
            let actual = target(root);
            assert_that!(actual).named(&name).is_equal_to(expected);
        });
    }

}

use leet_code::tree::TreeNode;
