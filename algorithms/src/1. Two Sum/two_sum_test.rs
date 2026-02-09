use super::*;

#[test]
fn two_sum_case1() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let expected = vec![0, 1];
    assert_eq!(Solution::two_sum(nums, target), expected);
}

#[test]
fn two_sum_case2() {
    let nums = vec![3, 2, 4];
    let target = 6;
    let expected = vec![1, 2];
    assert_eq!(Solution::two_sum(nums, target), expected);
}

#[test]
fn two_sum_case3() {
    let nums = vec![3, 3];
    let target = 6;
    let expected = vec![0, 1];
    assert_eq!(Solution::two_sum(nums, target), expected);
}
