use super::*;

#[test]
fn is_subsequence_case1() {
    let s = String::from("abc");
    let t = String::from("ahbgdc");
    let expected = true;
    assert_eq!(Solution::is_subsequence(s, t), expected);
}

#[test]
fn is_subsequence_case2() {
    let s = String::from("axc");
    let t = String::from("ahbgdc");
    let expected = false;
    assert_eq!(Solution::is_subsequence(s, t), expected);
}
