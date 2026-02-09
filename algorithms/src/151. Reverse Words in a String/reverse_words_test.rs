use super::*;

#[test]
fn reverse_words_case1() {
    let s = String::from("the sky is blue");
    let expected = String::from("blue is sky the");
    assert_eq!(Solution::reverse_words(s), expected);
}

#[test]
fn reverse_words_case2() {
    let s = String::from("  hello world  ");
    let expected = String::from("world hello");
    assert_eq!(Solution::reverse_words(s), expected);
}

#[test]
fn reverse_words_case3() {
    let s = String::from("a good   example");
    let expected = String::from("example good a");
    assert_eq!(Solution::reverse_words(s), expected);
}
