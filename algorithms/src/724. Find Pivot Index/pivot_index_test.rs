use super::*;

#[test]
fn pivot_index_case1() {
    let nums = vec![1, 7, 3, 6, 5, 6];
    let expected = 3;
    assert_eq!(Solution::pivot_index(nums), expected);
}

#[test]
fn pivot_index_case2() {
    let nums = vec![1, 2, 3];
    let expected = -1;
    assert_eq!(Solution::pivot_index(nums), expected);
}

#[test]
fn pivot_index_case3() {
    let nums = vec![2, 1, -1];
    let expected = 0;
    assert_eq!(Solution::pivot_index(nums), expected);
}
