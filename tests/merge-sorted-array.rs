//! https://leetcode.com/problems/merge-sorted-array

use spectral::prelude::*;

/// Merge nums1 and nums2 into a single array sorted in non-decreasing order.
/// You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, and two integers m and n, representing the number of elements in nums1 and nums2 respectively.
/// The final sorted array should not be returned by the function, but instead be stored inside the array nums1.
/// To accommodate this, nums1 has a length of m + n, where the first m elements denote the elements that should be merged, and the last n elements are set to 0 and should be ignored.
/// nums2 has a length of n.
pub fn merge(nums1: &mut Vec<i32>, mut m: i32, nums2: &mut Vec<i32>, mut n: i32) {
    if n == 0 {
        return;
    }

    if m == 0 {
        std::mem::swap(nums1, nums2);
        return;
    }

    let mut i = (m + n - 1) as usize;

    while m != 0 && n != 0 {
        let mi = (m - 1) as usize;
        let ni = (n - 1) as usize;

        if nums1[mi] > nums2[ni] {
            nums1[i] = nums1[mi];
            m = mi as i32;
        } else if nums1[mi] == nums2[ni] {
            nums1[i] = nums1[mi];
            m -= 1;
            i -= 1;
            nums1[i] = nums2[ni];
            n = ni as i32;
        } else {
            nums1[i] = nums2[ni];
            n -= 1;
        }

        i -= 1;
    }

    for j in 0..n {
        let j = j as usize;
        nums1[j] = nums2[j]
    }
}

#[test]
fn test() {
    let mut cases = [
        (
            vec![1, 2, 3], //
            vec![2, 5, 6],
            vec![1, 2, 2, 3, 5, 6],
        ),
        (
            vec![1, 2, 3],
            vec![4, 5, 6], //
            vec![1, 2, 3, 4, 5, 6],
        ),
        (
            vec![4, 5, 6], //
            vec![1, 2, 3],
            vec![1, 2, 3, 4, 5, 6],
        ),
        (
            vec![1, 1, 1], //
            vec![1, 1],
            vec![1, 1, 1, 1, 1],
        ),
        (
            vec![], //
            vec![1, 2, 3],
            vec![1, 2, 3],
        ),
        (
            vec![2, 5, 6], //
            vec![],
            vec![2, 5, 6],
        ),
        (vec![1], vec![2], vec![1, 2]),
        (vec![2], vec![1], vec![1, 2]),
        (vec![], vec![], vec![]),
        (
            vec![5, 6, 7], //
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4, 5, 6, 7],
        ),
    ];

    cases.iter_mut().for_each(|(nums1, nums2, expected)| {
        let case = format!("{nums1:?} + {nums2:?}");
        let m = nums1.len();
        nums1.extend(std::iter::repeat_n(0, nums2.len()));
        merge(nums1, m as i32, nums2, nums2.len() as i32);
        assert_that!(nums1).named(&case).is_equal_to(expected);
    });
}
