use super::*;

#[test]
fn find_difference() {
    let mut result = Solution::find_difference(vec![1, 2, 3], vec![2, 4, 6]);
    result.iter_mut().for_each(|v| v.sort());
    assert_eq!(result, vec![vec![1, 3], vec![4, 6]]);

    assert_eq!(
        Solution::find_difference(vec![1, 2, 3, 3], vec![1, 1, 2, 2]),
        vec![vec![3], vec![]]
    );
}
