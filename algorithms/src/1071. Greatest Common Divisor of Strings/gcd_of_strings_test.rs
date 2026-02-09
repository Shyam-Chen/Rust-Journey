use super::*;

#[test]
fn gcd_of_strings_case1() {
    let str1 = String::from("ABCABC");
    let str2 = String::from("ABC");
    let expected = String::from("ABC");
    assert_eq!(Solution::gcd_of_strings(str1, str2), expected);
}

#[test]
fn gcd_of_strings_case2() {
    let str1 = String::from("ABABAB");
    let str2 = String::from("ABAB");
    let expected = String::from("AB");
    assert_eq!(Solution::gcd_of_strings(str1, str2), expected);
}

#[test]
fn gcd_of_strings_case3() {
    let str1 = String::from("LEET");
    let str2 = String::from("CODE");
    let expected = String::from("");
    assert_eq!(Solution::gcd_of_strings(str1, str2), expected);
}
