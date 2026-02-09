//! https://leetcode.com/problems/merge-two-sorted-lists
//!
//! You are given the heads of two sorted linked lists list1 and list2.
//! Merge the two lists into one sorted list. The list should be made by splicing together the nodes of the first two lists.
//! Return the head of the merged linked list.
//!
//! Constraints:
//! - The number of nodes in both lists is in the range [0, 50].
//! - -100 <= Node.val <= 100
//! - Both list1 and list2 are sorted in non-decreasing order.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn merge_two_lists(
    _list1: Option<Box<ListNode>>,
    _list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn merge_two_lists_test() {
        let cases = vec![
            (vec![1, 2, 4], vec![1, 3, 4], vec![1, 1, 2, 3, 4, 4]), // Example 1
            (vec![], vec![], vec![]),                               // Example 2: both empty
            (vec![], vec![0], vec![0]),                             // Example 3: first empty
        ];

        cases.into_iter().for_each(|(list1, list2, expected)| {
            let name = format!("{list1:?} + {list2:?}");
            let list1 = vec_to_list(list1);
            let list2 = vec_to_list(list2);
            let expected = vec_to_list(expected);
            let actual = merge_two_lists(list1, list2);
            assert_that!(actual).named(&name).is_equal_to(expected);
        });
    }

    fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in vec.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }
}
