use super::*;

#[test]
fn can_place_flowers_case1() {
    let flowerbed = vec![1, 0, 0, 0, 1];
    let n = 1;
    let expected = true;
    assert_eq!(Solution::can_place_flowers(flowerbed, n), expected);
}

#[test]
fn can_place_flowers_case2() {
    let flowerbed = vec![1, 0, 0, 0, 1];
    let n = 2;
    let expected = false;
    assert_eq!(Solution::can_place_flowers(flowerbed, n), expected);
}
