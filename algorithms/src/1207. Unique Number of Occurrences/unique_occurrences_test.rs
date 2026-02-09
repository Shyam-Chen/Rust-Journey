use super::*;

#[test]
fn unique_occurrences_case1() {
    let arr = vec![1, 2, 2, 1, 1, 3];
    let expected = true;
    assert_eq!(Solution::unique_occurrences(arr), expected);
}

#[test]
fn unique_occurrences_case2() {
    let arr = vec![1, 2];
    let expected = false;
    assert_eq!(Solution::unique_occurrences(arr), expected);
}

#[test]
fn unique_occurrences_case3() {
    let arr = vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0];
    let expected = true;
    assert_eq!(Solution::unique_occurrences(arr), expected);
}
