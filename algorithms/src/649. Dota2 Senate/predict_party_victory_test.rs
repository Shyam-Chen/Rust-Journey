use super::*;

#[test]
fn predict_party_victory_case1() {
    let senate = String::from("RD");
    let expected = String::from("Radiant");
    assert_eq!(Solution::predict_party_victory(senate), expected);
}

#[test]
fn predict_party_victory_case2() {
    let senate = String::from("RDD");
    let expected = String::from("Dire");
    assert_eq!(Solution::predict_party_victory(senate), expected);
}
