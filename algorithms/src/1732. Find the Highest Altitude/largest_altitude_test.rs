use super::*;

#[test]
fn largest_altitude_case1() {
    let gain = vec![-5, 1, 5, 0, -7];
    let expected = 1;
    assert_eq!(Solution::largest_altitude(gain), expected);
}

#[test]
fn largest_altitude_case2() {
    let gain = vec![-4, -3, -2, -1, 4, 3, 2];
    let expected = 0;
    assert_eq!(Solution::largest_altitude(gain), expected);
}
