use super::*;

#[test]
fn merge_alternately_case1() {
    let word1 = String::from("abc");
    let word2 = String::from("pqr");
    let expected = String::from("apbqcr");
    assert_eq!(Solution::merge_alternately(word1, word2), expected);
}

#[test]
fn merge_alternately_case2() {
    let word1 = String::from("ab");
    let word2 = String::from("pqrs");
    let expected = String::from("apbqrs");
    assert_eq!(Solution::merge_alternately(word1, word2), expected);
}

#[test]
fn merge_alternately_case3() {
    let word1 = String::from("abcd");
    let word2 = String::from("pq");
    let expected = String::from("apbqcd");
    assert_eq!(Solution::merge_alternately(word1, word2), expected);
}
