use super::*;

#[test]
fn product_except_self_case1() {
    let nums = vec![1, 2, 3, 4];
    let expected = vec![24, 12, 8, 6];
    assert_eq!(Solution::product_except_self(nums), expected);
}

#[test]
fn product_except_self_case2() {
    let nums = vec![-1, 1, 0, -3, 3];
    let expected = vec![0, 0, 9, 0, 0];
    assert_eq!(Solution::product_except_self(nums), expected);
}
