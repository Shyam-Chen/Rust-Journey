use super::*;

#[test]
fn recent_counter() {
    let mut recent_counter = RecentCounter::new();
    assert_eq!(recent_counter.ping(1), 1);
    assert_eq!(recent_counter.ping(100), 2);
    assert_eq!(recent_counter.ping(3001), 3);
    assert_eq!(recent_counter.ping(3002), 3);
}
