use super::*;

use crate::utils::list_node::{from_vec, to_vec};

#[test]
fn delete_middle_case1() {
    let head = from_vec(&[1, 3, 4, 7, 1, 2, 6]);
    let result = Solution::delete_middle(head);
    let expected = vec![1, 3, 4, 1, 2, 6];
    assert_eq!(to_vec(result), expected);
}

#[test]
fn delete_middle_case2() {
    let head = from_vec(&[1, 2, 3, 4]);
    let result = Solution::delete_middle(head);
    let expected = vec![1, 2, 4];
    assert_eq!(to_vec(result), expected);
}

#[test]
fn delete_middle_case3() {
    let head = from_vec(&[2, 1]);
    let result = Solution::delete_middle(head);
    let expected = vec![2];
    assert_eq!(to_vec(result), expected);
}
