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

    pub fn build(values: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
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
