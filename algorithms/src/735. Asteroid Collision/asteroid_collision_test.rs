use super::*;

#[test]
fn asteroid_collision_case1() {
    let asteroids = vec![5, 10, -5];
    let expected = vec![5, 10];
    assert_eq!(Solution::asteroid_collision(asteroids), expected);
}

#[test]
fn asteroid_collision_case2() {
    let asteroids = vec![8, -8];
    let expected = vec![];
    assert_eq!(Solution::asteroid_collision(asteroids), expected);
}

#[test]
fn asteroid_collision_case3() {
    let asteroids = vec![10, 2, -5];
    let expected = vec![10];
    assert_eq!(Solution::asteroid_collision(asteroids), expected);
}

#[test]
fn asteroid_collision_case4() {
    let asteroids = vec![3, 5, -6, 2, -1, 4];
    let expected = vec![-6, 2, 4];
    assert_eq!(Solution::asteroid_collision(asteroids), expected);
}
