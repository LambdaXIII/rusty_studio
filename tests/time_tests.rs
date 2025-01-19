use rstu::core::Time;
use rstu::core::Timebase;

#[test]
fn test_from_millisecond() {
    let time = Time::from_millisecond(1234);
    assert_eq!(time.to_millisecond(), 1234);
}

#[test]
fn test_from_seconds() {
    let time = Time::from_seconds(&1.234);
    assert_eq!(time.to_millisecond(), 1234);
}

#[test]
fn test_from_timecode() {
    let timebase = Timebase { fps: 24, drop_frame: false };
    let time = Time::from_timecode("00:00:01:00", &timebase).unwrap();
    assert_eq!(time.to_millisecond(), 1000);
}

#[test]
fn test_to_timecode() {
    let timebase = Timebase { fps: 24, drop_frame: false };
    let time = Time::from_millisecond(1000);
    assert_eq!(time.to_timecode(&timebase), "00:00:01:00");
}

#[test]
fn test_from_timestamp() {
    let time = Time::from_timestamp("00:00:01.000").unwrap();
    assert_eq!(time.to_millisecond(), 1000);
}

#[test]
fn test_to_timestamp() {
    let time = Time::from_millisecond(1000);
    assert_eq!(time.to_timestamp(), "00:00:01.000");
}

#[test]
fn test_add() {
    let time1 = Time::from_millisecond(1000);
    let time2 = Time::from_millisecond(2000);
    let result = time1 + time2;
    assert_eq!(result.to_millisecond(), 3000);
}

#[test]
fn test_sub() {
    let time1 = Time::from_millisecond(3000);
    let time2 = Time::from_millisecond(2000);
    let result = time1 - time2;
    assert_eq!(result.to_millisecond(), 1000);
}

#[test]
fn test_mul() {
    let time = Time::from_millisecond(1000);
    let result = time * 2.0;
    assert_eq!(result.to_millisecond(), 2000);
}

#[test]
fn test_div() {
    let time = Time::from_millisecond(2000);
    let result = time / 2.0;
    assert_eq!(result.to_millisecond(), 1000);
}

#[test]
fn test_eq() {
    let time1 = Time::from_millisecond(1000);
    let time2 = Time::from_millisecond(1000);
    assert_eq!(time1, time2);
}

#[test]
fn test_ne() {
    let time1 = Time::from_millisecond(1000);
    let time2 = Time::from_millisecond(2000);
    assert_ne!(time1, time2);
}

#[test]
fn test_partial_cmp() {
    let time1 = Time::from_millisecond(1000);
    let time2 = Time::from_millisecond(2000);
    assert!(time1 < time2);
    assert!(time2 > time1);
}

#[test]
fn test_cmp() {
    let time1 = Time::from_millisecond(1000);
    let time2 = Time::from_millisecond(2000);
    assert_eq!(time1.cmp(&time2), std::cmp::Ordering::Less);
    assert_eq!(time2.cmp(&time1), std::cmp::Ordering::Greater);
}

#[test]
fn test_hash() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let time1 = Time::from_millisecond(1000);
    let time2 = Time::from_millisecond(1000);

    let mut hasher1 = DefaultHasher::new();
    time1.hash(&mut hasher1);
    let hash1 = hasher1.finish();

    let mut hasher2 = DefaultHasher::new();
    time2.hash(&mut hasher2);
    let hash2 = hasher2.finish();

    assert_eq!(hash1, hash2);
}