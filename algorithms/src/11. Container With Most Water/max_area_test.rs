use super::*;

#[test]
fn max_area_case1() {
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let expected = 49;
    assert_eq!(Solution::max_area(height), expected);
}

#[test]
fn max_area_case2() {
    let height = vec![1, 1];
    let expected = 1;
    assert_eq!(Solution::max_area(height), expected);
}
