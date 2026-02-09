use super::*;

#[test]
fn find_difference_case1() {
    let nums1 = vec![1, 2, 3];
    let nums2 = vec![2, 4, 6];
    let mut result = Solution::find_difference(nums1, nums2);
    result.iter_mut().for_each(|v| v.sort());
    let expected = vec![vec![1, 3], vec![4, 6]];
    assert_eq!(result, expected);
}

#[test]
fn find_difference_case2() {
    let nums1 = vec![1, 2, 3, 3];
    let nums2 = vec![1, 1, 2, 2];
    let mut result = Solution::find_difference(nums1, nums2);
    result.iter_mut().for_each(|v| v.sort());
    let expected = vec![vec![3], vec![]];
    assert_eq!(result, expected);
}
