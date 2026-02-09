use super::*;

#[test]
fn find_max_average_case1() {
    let nums = vec![1, 12, -5, -6, 50, 3];
    let k = 4;
    let expected = 12.75000;
    assert_eq!(Solution::find_max_average(nums, k), expected);
}

#[test]
fn find_max_average_case2() {
    let nums = vec![5];
    let k = 1;
    let expected = 5.00000;
    assert_eq!(Solution::find_max_average(nums, k), expected);
}
