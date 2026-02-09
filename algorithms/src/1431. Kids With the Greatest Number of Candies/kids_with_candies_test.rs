use super::*;

#[test]
fn kids_with_candies_case1() {
    let candies = vec![2, 3, 5, 1, 3];
    let extra_candies = 3;
    let expected = vec![true, true, true, false, true];
    assert_eq!(
        Solution::kids_with_candies(candies, extra_candies),
        expected
    );
}

#[test]
fn kids_with_candies_case2() {
    let candies = vec![4, 2, 1, 1, 2];
    let extra_candies = 1;
    let expected = vec![true, false, false, false, false];
    assert_eq!(
        Solution::kids_with_candies(candies, extra_candies),
        expected
    );
}

#[test]
fn kids_with_candies_case3() {
    let candies = vec![12, 1, 12];
    let extra_candies = 10;
    let expected = vec![true, false, true];
    assert_eq!(
        Solution::kids_with_candies(candies, extra_candies),
        expected
    );
}
