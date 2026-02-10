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

use leet_code::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

// 0ms | 3MB - 98.04% | DFS | O(n log n)
pub fn kth_smallest_1(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    let mut stack = std::collections::VecDeque::new();
    let mut values = vec![];
    let node = root.unwrap();
    stack.push_back(node);

    while let Some(node) = stack.pop_front() {
        if let Some(left) = node.borrow_mut().left.take() {
            stack.push_back(left);
        };

        if let Some(right) = node.borrow_mut().right.take() {
            stack.push_back(right);
        };

        values.push(node.borrow().val);
    }

    values.sort_unstable();
    values[k as usize - 1]
}

// 0ms | 2.99MB - 98.08% | work, but cumbersome
pub fn kth_smallest_2(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    let mut stack = Vec::new();
    let mut cnt = 0;
    let node = root.unwrap();
    stack.push(node);

    while let Some(node) = stack.pop() {
        if let Some(right) = node.borrow_mut().right.take() {
            stack.push(right);
        }

        if let Some(left) = node.borrow_mut().left.take() {
            // parent node need to go in the middle to be able to access its value.
            // it's safe to reiterate it as child nodes are "taken" out at that moment.
            stack.push(node.clone());
            stack.push(left);
        } else {
            cnt += 1;
            if cnt == k {
                return node.borrow().val;
            }
        }
    }

    -1
}

// 0ms | 2.92MB - 98% | not optimal, as collects all values
pub fn kth_smallest_3(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    fn traverse(node: Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>) {
        let Some(node) = node else {
            return;
        };

        let val = node.borrow().val;
        traverse(node.borrow_mut().left.take(), values);
        values.push(val);
        traverse(node.borrow_mut().right.take(), values);
    }

    let mut values = vec![];
    traverse(root, &mut values);
    values[k as usize - 1]
}

// still is not optimal, as collects >= k values and needs to unwind the recursion
pub fn kth_smallest_4(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    fn traverse(node: Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>, k: usize) {
        let Some(node) = node else {
            return;
        };

        traverse(node.borrow_mut().left.take(), values, k);

        let val = node.borrow().val;
        values.push(val);

        if values.len() == k {
            return;
        }

        traverse(node.borrow_mut().right.take(), values, k);
    }

    let k = k as usize;
    let mut values = vec![];
    traverse(root, &mut values, k);
    values[k - 1]
}

// optimal recursive solution
pub fn kth_smallest_5(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    fn traverse(node: Option<Rc<RefCell<TreeNode>>>, cnt: &mut i32, k: i32) -> Option<i32> {
        let node = node?;

        if let Some(val) = traverse(node.borrow_mut().left.take(), cnt, k) {
            return Some(val);
        }

        let val = node.borrow().val;
        *cnt += 1;
        if cnt == &k {
            return Some(val);
        }

        if let Some(val) = traverse(node.borrow_mut().right.take(), cnt, k) {
            return Some(val);
        }

        None
    }

    traverse(root, &mut 0, k).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use leet_code::solution_tests;
    use spectral::prelude::*;

    solution_tests!(
        run_test:
        kth_smallest_1,
        kth_smallest_2,
        kth_smallest_3,
        kth_smallest_4,
        kth_smallest_5,
    );

    fn run_test(target: fn(Option<Rc<RefCell<TreeNode>>>, i32) -> i32) {
        vec![
            (vec![Some(3), Some(1), Some(4), None, Some(2)], 1, 1),
            (vec![Some(2), Some(1), Some(3)], 3, 3),
            (vec![Some(1), None, Some(2)], 2, 2),
            (vec![Some(3), Some(1), Some(4), None, Some(2)], 2, 2),
            (vec![Some(1)], 1, 1),
            (
                vec![
                    Some(5),
                    Some(3),
                    Some(6),
                    Some(2),
                    Some(4),
                    None,
                    None,
                    Some(1),
                ],
                3,
                3,
            ),
        ]
        .into_iter()
        .for_each(|(tree, k, expected)| {
            let root = TreeNode::build(&tree);
            let name = format!("tree={tree:?}, k={k}");
            let actual = target(root, k);
            assert_that!(actual).named(&name).is_equal_to(expected);
        });
    }
}
