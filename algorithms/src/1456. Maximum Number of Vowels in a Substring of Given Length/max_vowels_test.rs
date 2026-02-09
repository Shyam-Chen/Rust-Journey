use super::*;

#[test]
fn max_vowels_case1() {
    let s = String::from("abciiidef");
    let k = 3;
    let expected = 3;
    assert_eq!(Solution::max_vowels(s, k), expected);
}

#[test]
fn max_vowels_case2() {
    let s = String::from("aeiou");
    let k = 2;
    let expected = 2;
    assert_eq!(Solution::max_vowels(s, k), expected);
}

#[test]
fn max_vowels_case3() {
    let s = String::from("leetcode");
    let k = 3;
    let expected = 2;
    assert_eq!(Solution::max_vowels(s, k), expected);
}
