//! <https://leetcode.com/problems/maximum-depth-of-binary-tree/>
//!
//! Given the root of a binary tree, return its maximum depth.
//!
//! A binary tree's maximum depth is the number of nodes along the longest path from the root node
//! down to the farthest leaf node.
//!
//! ## Constraints:
//! - The number of nodes in the tree is in the range `[0, 10^4]`
//! - `-100 <= Node.val <= 100`

use std::cell::RefCell;
use std::rc::Rc;
use leet_code::tree::{TreeNode, build_tree};

// 0ms | 2.69MB - 74.3%
pub fn max_depth_1(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let Some(root) = root else {
        return 0;
    };

    let l = max_depth_1(root.borrow().left.clone());
    let r = max_depth_1(root.borrow().right.clone());
    l.max(r) + 1
}

// 0ms | 2.61MB - 73.71%
// stack gives DFS
// queue gives BFS
pub fn max_depth_2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let Some(root) = root else {
        return 0;
    };

    let mut stack = vec![];
    stack.push((root.clone(), 1));
    let mut depth = 0;

    while let Some((node, d)) = stack.pop() {
        depth = d.max(depth);

        if let Some(l) = node.borrow().left.clone() {
            stack.push((l, d + 1));
        }

        if let Some(r) = node.borrow().right.clone() {
            stack.push((r, d + 1));
        }
    }

    depth
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        max_depth_1,
        max_depth_2,
    );

    fn run_test(target: fn(Option<Rc<RefCell<TreeNode>>>) -> i32) {
        vec![
            (
                vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
                3,
            ), // Example 1: complete tree
            (vec![Some(1), None, Some(2)], 2), // Example 2: right-skewed tree
            (vec![], 0),                       // Empty tree
        ]
        .into_iter()
        .for_each(|(input, expected)| {
            let root = build_tree(&input);
            let name = format!("max_depth({:?}) = {}", input, expected);
            let actual = target(root);
            assert_that!(actual).named(&name).is_equal_to(expected);
        });
    }

}
