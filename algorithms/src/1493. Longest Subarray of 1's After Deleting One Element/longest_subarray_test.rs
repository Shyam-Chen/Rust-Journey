use super::*;

#[test]
fn longest_subarray_case1() {
    let nums = vec![1, 1, 0, 1];
    let expected = 3;
    assert_eq!(Solution::longest_subarray(nums), expected);
}

#[test]
fn longest_subarray_case2() {
    let nums = vec![0, 1, 1, 1, 0, 1, 1, 0, 1];
    let expected = 5;
    assert_eq!(Solution::longest_subarray(nums), expected);
}

#[test]
fn longest_subarray_case3() {
    let nums = vec![1, 1, 1];
    let expected = 2;
    assert_eq!(Solution::longest_subarray(nums), expected);
}
