use super::*;

#[test]
fn max_operations_case1() {
    let nums = vec![1, 2, 3, 4];
    let k = 5;
    let expected = 2;
    assert_eq!(Solution::max_operations(nums, k), expected);
}

#[test]
fn max_operations_case2() {
    let nums = vec![3, 1, 3, 4, 3];
    let k = 6;
    let expected = 1;
    assert_eq!(Solution::max_operations(nums, k), expected);
}
