use super::*;

#[test]
fn remove_stars_case1() {
    let s = String::from("leet**cod*e");
    let expected = String::from("lecoe");
    assert_eq!(Solution::remove_stars(s), expected);
}

#[test]
fn remove_stars_case2() {
    let s = String::from("erase*****");
    let expected = String::from("");
    assert_eq!(Solution::remove_stars(s), expected);
}
