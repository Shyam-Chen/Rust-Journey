use super::*;

#[test]
fn close_strings_case1() {
    let word1 = String::from("abc");
    let word2 = String::from("bca");
    let expected = true;
    assert_eq!(Solution::close_strings(word1, word2), expected);
}

#[test]
fn close_strings_case2() {
    let word1 = String::from("a");
    let word2 = String::from("aa");
    let expected = false;
    assert_eq!(Solution::close_strings(word1, word2), expected);
}

#[test]
fn close_strings_case3() {
    let word1 = String::from("cabbba");
    let word2 = String::from("abbccc");
    let expected = true;
    assert_eq!(Solution::close_strings(word1, word2), expected);
}
