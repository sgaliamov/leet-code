/*
You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, and two integers m and n, representing the number of elements in nums1 and nums2 respectively.
Merge nums1 and nums2 into a single array sorted in non-decreasing order.
The final sorted array should not be returned by the function, but instead be stored inside the array nums1. To accommodate this, nums1 has a length of m + n, where the first m elements denote the elements that should be merged, and the last n elements are set to 0 and should be ignored. nums2 has a length of n.
*/

use std::{iter, mem};

use spectral::assert_that;

pub fn merge(nums1: &mut Vec<i32>, mut m: usize, nums2: &mut Vec<i32>, mut n: usize) {
    let mut i = nums1.len() - 1;
    assert!(nums1.iter().take(m).is_sorted(), "{:?}", nums1);
    assert!(nums2.is_sorted());

    if m == 0 {
        mem::swap(nums1, nums2);
        return;
    }

    while n != 0 {
        if nums1[m - 1] > nums2[n - 1] {
            nums1[i] = nums1[m - 1];
            m -= 1;
        } else if nums1[m - 1] == nums2[n - 1] {
            nums1[i] = nums1[m - 1];
            m -= 1;
            i -= 1;
            nums1[i] = nums2[n - 1];
            n -= 1;
        } else if nums1[m - 1] < nums2[n - 1] {
            nums1[i] = nums2[n - 1];
            n -= 1;
        }

        i -= 1;
    }
}

#[test]
fn test() {
    let mut cases = [
        // (vec![1, 2, 3], vec![2, 5, 6], vec![1, 2, 2, 3, 5, 6]),
        (vec![2, 3, 5], vec![1, 5], vec![1, 2, 3, 5, 5]),
        // (vec![1, 1, 1], vec![1, 1], vec![1, 1, 1, 1, 1]),
        // (vec![], vec![1, 2, 3], vec![1, 2, 3]),
        // (vec![2, 5, 6], vec![], vec![2, 5, 6]),
        // (vec![1], vec![2], vec![1, 2]),
        (vec![2], vec![1], vec![1, 2]),
    ];

    cases.iter_mut().for_each(|(nums1, nums2, expected)| {
        let m = nums1.len();
        nums1.extend(iter::repeat_n(0, nums2.len()));
        merge(nums1, m, nums2, nums2.len());
        assert_that!(nums1).is_equal_to(expected);
    });
}
