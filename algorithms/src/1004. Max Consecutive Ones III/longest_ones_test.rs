use super::*;

#[test]
fn longest_ones_case1() {
    let nums = vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0];
    let k = 2;
    let expected = 6;
    assert_eq!(Solution::longest_ones(nums, k), expected);
}

#[test]
fn longest_ones_case2() {
    let nums = vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1];
    let k = 3;
    let expected = 10;
    assert_eq!(Solution::longest_ones(nums, k), expected);
}
