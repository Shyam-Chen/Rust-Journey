use super::*;

#[test]
fn increasing_triplet_case1() {
    let nums = vec![1, 2, 3, 4, 5];
    let expected = true;
    assert_eq!(Solution::increasing_triplet(nums), expected);
}

#[test]
fn increasing_triplet_case2() {
    let nums = vec![5, 4, 3, 2, 1];
    let expected = false;
    assert_eq!(Solution::increasing_triplet(nums), expected);
}

#[test]
fn increasing_triplet_case3() {
    let nums = vec![2, 1, 5, 0, 4, 6];
    let expected = true;
    assert_eq!(Solution::increasing_triplet(nums), expected);
}
