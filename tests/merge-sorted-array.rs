/*
You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, and two integers m and n, representing the number of elements in nums1 and nums2 respectively.
Merge nums1 and nums2 into a single array sorted in non-decreasing order.
The final sorted array should not be returned by the function, but instead be stored inside the array nums1. To accommodate this, nums1 has a length of m + n, where the first m elements denote the elements that should be merged, and the last n elements are set to 0 and should be ignored. nums2 has a length of n.
*/

use spectral::assert_that;

/// https://leetcode.com/problems/merge-sorted-array
pub fn merge(nums1: &mut Vec<i32>, mut m: i32, nums2: &mut Vec<i32>, mut n: i32) {
    use std::mem;
    assert!(nums1.iter().take(m as usize).is_sorted(), "{:?}", nums1);
    assert!(nums2.is_sorted());

    if n == 0 {
        return;
    }

    if m == 0 {
        mem::swap(nums1, nums2);
        return;
    }

    let mut i = nums1.len() - 1;

    while i != 0 {
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
            n -= 1;
        } else if nums1[mi] < nums2[ni] {
            nums1[i] = nums2[ni];
            n -= 1;
        }

        if n == 0 {
            break;
        }

        if m == 0 {
            for j in 0..n {
                let j = j as usize;
                nums1[j] = nums2[j]
            }
            break;
        }

        i -= 1;
    }

    // we cannot continue to decrease `m` and `n` as they will become negative,
    // therefore, we have to check the last element directly:
    if nums1[0] > nums2[0] {
        nums1[0] = nums2[0];
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
    ];

    cases.iter_mut().for_each(|(nums1, nums2, expected)| {
        let case = format!("{nums1:?} + {nums2:?}");
        let m = nums1.len();
        nums1.extend(std::iter::repeat_n(0, nums2.len()));
        merge(nums1, m as i32, nums2, nums2.len() as i32);
        assert_that!(nums1).named(&case).is_equal_to(expected);
    });
}
