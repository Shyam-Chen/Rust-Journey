use super::*;

#[test]
fn equal_pairs_case1() {
    let grid = vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]];
    let expected = 1;
    assert_eq!(Solution::equal_pairs(grid), expected);
}

#[test]
fn equal_pairs_case2() {
    let grid = vec![
        vec![3, 1, 2, 2],
        vec![1, 4, 4, 5],
        vec![2, 4, 2, 2],
        vec![2, 4, 2, 2],
    ];
    let expected = 3;
    assert_eq!(Solution::equal_pairs(grid), expected);
}
