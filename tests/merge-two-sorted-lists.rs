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

// 0ms | 2.00MB
pub fn merge_two_lists_1(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut values = vec![];

    loop {
        let Some(mut l1) = list1.take() else {
            while let Some(node) = list2 {
                values.push(node.val);
                list2 = node.next;
            }
            break;
        };

        let Some(mut l2) = list2.take() else {
            values.push(l1.val);
            while let Some(node) = l1.next {
                values.push(node.val);
                l1 = node
            }
            break;
        };

        if l1.val < l2.val {
            values.push(l1.val);

            if let Some(next) = l1.next.take() {
                list1 = Some(next);
            }

            list2 = Some(l2);
        } else {
            values.push(l2.val);

            if let Some(next) = l2.next.take() {
                list2 = Some(next);
            }

            list1 = Some(l1);
        }
    }

    let mut head = None;
    for val in values.into_iter().rev() {
        let mut node = ListNode::new(val);
        node.next = head;
        head = Some(Box::new(node));
    }

    head
}

pub struct ListIter<'a> {
    node: &'a Option<Box<ListNode>>,
}

impl<'a> ListIter<'a> {
    pub fn new(node: &'a Option<Box<ListNode>>) -> Self {
        Self { node }
    }
}

impl<'a> Iterator for ListIter<'a>
where
    Self: 'a,
{
    type Item = &'a Box<ListNode>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(r) = self.node {
            self.node = &r.next;
            return Some(r);
        }

        None
    }
}

pub fn merge_two_lists_2(
    mut _list1: Option<Box<ListNode>>,
    mut _list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    todo!("solve")
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn merge_two_lists_test() {
        let cases = vec![
            (
                vec![-10, -6, -6, -6, -3, 5],
                vec![],
                vec![-10, -6, -6, -6, -3, 5],
            ),
            (vec![5], vec![1, 2, 4], vec![1, 2, 4, 5]),
            (vec![1, 3, 5], vec![2, 4], vec![1, 2, 3, 4, 5]),
            (vec![1, 2, 4], vec![1, 3, 4], vec![1, 1, 2, 3, 4, 4]),
            (vec![1, 2], vec![1, 3, 4], vec![1, 1, 2, 3, 4]),
            (vec![], vec![], vec![]),
            (vec![], vec![0], vec![0]),
            (vec![0], vec![], vec![0]),
        ];

        cases.into_iter().for_each(|(list1, list2, expected)| {
            let name = format!("{list1:?} + {list2:?}");
            let list1 = vec_to_list(list1);
            let list2 = vec_to_list(list2);
            let expected = vec_to_list(expected);
            let actual = merge_two_lists_2(list1, list2);
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
