use rusty_studio::core::{MetadataSupport, Time};
use rusty_studio::timeline::{ContentSupport, Item, TimeRangeEditableTrait, TimeRangeTrait};

#[test]
fn test_item_new() {
    let item = Item::new();
    assert_eq!(item.start(), Time::new(0));
    assert_eq!(item.duration(), Time::new(0));
    assert!(item.get_content::<String>().is_none());
    assert!(item
        .get_metadata::<String>(&"test_key".to_string())
        .is_none());
}

#[test]
fn test_item_from_timerange() {
    let range = TimeRange {
        start: Time::new(10),
        duration: Time::new(20),
    };
    let item = Item::from_timerange(range);
    assert_eq!(item.start(), Time::new(10));
    assert_eq!(item.duration(), Time::new(20));
}

#[test]
fn test_item_metadata() {
    let mut item = Item::new();
    let mut metadata = item.metadata();
    metadata.set(&*"test_key".to_string(), "test_value".to_string());
    drop(metadata);
    assert_eq!(
        item.get_metadata::<String>(&"test_key".to_string()),
        Some("test_value".to_string())
    );
}

#[test]
fn test_item_content() {
    let mut item = Item::new();
    item.set_content("test_content".to_string());
    assert_eq!(
        item.get_content::<String>(),
        Some("test_content".to_string())
    );
    item.clear_content();
    assert!(item.get_content::<String>().is_none());
}

#[test]
fn test_item_time_range_editable() {
    let mut item = Item::new();
    item.set_start(Time::new(5));
    item.set_duration(Time::new(15));
    assert_eq!(item.start(), Time::new(5));
    assert_eq!(item.duration(), Time::new(15));
}

// 辅助结构体用于测试 from_timerange 方法
struct TimeRange {
    start: Time,
    duration: Time,
}

impl TimeRangeTrait for TimeRange {
    fn start(&self) -> Time {
        self.start
    }

    fn duration(&self) -> Time {
        self.duration
    }
}
