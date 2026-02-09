use super::*;

#[test]
fn compress_case1() {
    let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
    let expected_len = 6;
    let expected_chars = vec!['a', '2', 'b', '2', 'c', '3'];
    let len = Solution::compress(&mut chars);
    assert_eq!(len, expected_len);
    assert_eq!(&chars[..len as usize], expected_chars);
}

#[test]
fn compress_case2() {
    let mut chars = vec!['a'];
    let expected_len = 1;
    let expected_chars = vec!['a'];
    let len = Solution::compress(&mut chars);
    assert_eq!(len, expected_len);
    assert_eq!(&chars[..len as usize], expected_chars);
}

#[test]
fn compress_case3() {
    let mut chars = vec![
        'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
    ];
    let expected_len = 4;
    let expected_chars = vec!['a', 'b', '1', '2'];
    let len = Solution::compress(&mut chars);
    assert_eq!(len, expected_len);
    assert_eq!(&chars[..len as usize], expected_chars);
}
