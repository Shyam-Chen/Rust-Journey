use super::*;

#[test]
fn reverse_vowels_case1() {
    let s = String::from("IceCreAm");
    let expected = String::from("AceCreIm");
    assert_eq!(Solution::reverse_vowels(s), expected);
}

#[test]
fn reverse_vowels_case2() {
    let s = String::from("leetcode");
    let expected = String::from("leotcede");
    assert_eq!(Solution::reverse_vowels(s), expected);
}
