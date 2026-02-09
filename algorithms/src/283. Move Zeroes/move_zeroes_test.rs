use super::*;

#[test]
fn move_zeroes_case1() {
    let mut nums = vec![0, 1, 0, 3, 12];
    let expected = vec![1, 3, 12, 0, 0];
    Solution::move_zeroes(&mut nums);
    assert_eq!(nums, expected);
}

#[test]
fn move_zeroes_case2() {
    let mut nums = vec![0];
    let expected = vec![0];
    Solution::move_zeroes(&mut nums);
    assert_eq!(nums, expected);
}
