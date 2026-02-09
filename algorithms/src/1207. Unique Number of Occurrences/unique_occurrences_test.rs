use super::*;

#[test]
fn max_vowels() {
    assert_eq!(Solution::unique_occurrences(vec![1, 2, 2, 1, 1, 3]), true);
    assert_eq!(Solution::unique_occurrences(vec![1, 2]), false);
    assert_eq!(
        Solution::unique_occurrences(vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0]),
        true
    );
}
