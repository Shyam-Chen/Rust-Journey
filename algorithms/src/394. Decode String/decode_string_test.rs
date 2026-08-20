use super::*;

#[test]
fn decode_string_case1() {
    let s = String::from("3[a]2[bc]");
    let expected = String::from("aaabcbc");
    assert_eq!(Solution::decode_string(s), expected);
}

#[test]
fn decode_string_case2() {
    let s = String::from("3[a2[c]]");
    let expected = String::from("accaccacc");
    assert_eq!(Solution::decode_string(s), expected);
}

#[test]
fn decode_string_case3() {
    let s = String::from("2[abc]3[cd]ef");
    let expected = String::from("abcabccdcdcdef");
    assert_eq!(Solution::decode_string(s), expected);
}
