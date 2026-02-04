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

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let Some(root) = root else {
        return 0;
    };

    let l = max_depth(root.borrow().left.clone());
    let r = max_depth(root.borrow().right.clone());
    l.max(r) + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        max_depth,
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

    fn build_tree(values: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() || values[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(Rc::clone(&root));

        let mut i = 1;
        while i < values.len() {
            if let Some(node) = queue.pop_front() {
                if i < values.len() {
                    if let Some(val) = values[i] {
                        let left = Rc::new(RefCell::new(TreeNode::new(val)));
                        node.borrow_mut().left = Some(Rc::clone(&left));
                        queue.push_back(left);
                    }
                    i += 1;
                }

                if i < values.len() {
                    if let Some(val) = values[i] {
                        let right = Rc::new(RefCell::new(TreeNode::new(val)));
                        node.borrow_mut().right = Some(Rc::clone(&right));
                        queue.push_back(right);
                    }
                    i += 1;
                }
            }
        }

        Some(root)
    }
}
