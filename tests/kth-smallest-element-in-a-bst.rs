//! https://leetcode.com/problems/kth-smallest-element-in-a-bst/
//!
//! Given the root of a binary search tree, and an integer k, return the kth smallest value
//! (1-indexed) of all the values of the nodes in the tree.
//!
//! Constraints:
//! - The number of nodes in the tree is n
//! - 1 <= k <= n <= 10^4
//! - 0 <= Node.val <= 10^4
//!
//! Follow up: If the BST is modified often (i.e., we can do insert and delete operations)
//! and you need to find the kth smallest frequently, how would you optimize?

use std::cell::RefCell;
use std::rc::Rc;
use leet_code::tree::TreeNode;

pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        kth_smallest,
    );

    fn run_test(target: fn(Option<Rc<RefCell<TreeNode>>>, i32) -> i32) {
        vec![
            (vec![Some(3), Some(1), Some(4), None, Some(2)], 1, 1), // BST with 4 nodes, find 1st smallest
            (vec![Some(5), Some(3), Some(6), Some(2), Some(4), None, None, Some(1)], 3, 3), // BST with 7 nodes, find 3rd smallest
        ]
        .into_iter()
        .for_each(|(tree, k, expected)| {
            let root = build_tree(&tree);
            let name = format!("tree={tree:?}, k={k}");
            let actual = target(root, k);
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

