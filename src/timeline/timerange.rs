use super::{TimeRangeEditableTrait, TimeRangeTrait};
use crate::core::Time;

#[derive(Debug, Clone, Copy)]
pub struct TimeRange {
    start: Time,
    duration: Time,
}

impl TimeRange {
    pub fn new(start: Time, duration: Time) -> Self {
        Self { start, duration }
    }

    pub fn from_millisecond(start: i128, duration: i128) -> Self {
        Self {
            start: Time::from(start),
            duration: Time::from(duration),
        }
    }

    /**
    Construct Timerange from another struct implemented TimeRangeTrait.

    Example:
    ```rust
    # use rusty_studio::timeline::{Item,TimeRange,TimeRangeTrait};
    let range = TimeRange::from_millisecond(10,30);
    let item = Item::from_timerange(range);
    assert_eq!(item.start().to_millisecond(),10);
    assert_eq!(item.end().to_millisecond(),40);
    assert_eq!(item.duration().to_millisecond(),30);
    ```
    */
    pub fn from_timerange(range: &dyn TimeRangeTrait) -> Self {
        Self {
            start: range.start(),
            duration: range.duration(),
        }
    }

    /**
    Generate a time range contains the whole time range of given time ranges.

    `ranges` is an iterator of time ranges.

    Example:
    ```rust
    # use rusty_studio::timeline::{Item,TimeRange,TimeRangeTrait};
    # use rusty_studio::core::Time;
    let ranges = vec![
        TimeRange::from_millisecond(10,20),
        TimeRange::from_millisecond(20,30),
        TimeRange::from_millisecond(15,50)
    ];
    let whole_range = TimeRange::whole_timerange(&ranges);
    assert_eq!(whole_range.start().to_millisecond(),10);
    assert_eq!(whole_range.end().to_millisecond(),65);
    assert_eq!(whole_range.duration().to_millisecond(),55);
    ```
    */
    pub fn whole_timerange<I>(ranges: &Vec<I>) -> Self
    where
        I: TimeRangeTrait,
    {
        let mut start: Option<Time> = None;
        let mut end: Option<Time> = None;
        for r in ranges {
            if start.is_none() || start.unwrap() > r.start() {
                start = Some(r.start());
            }
            if end.is_none() || end.unwrap() < r.end() {
                end = Some(r.end());
            }
        }
        Self {
            start: start.unwrap(),
            duration: end.unwrap() - start.unwrap(),
        }
    }
}

impl TimeRangeTrait for TimeRange {
    fn start(&self) -> Time {
        self.start
    }

    fn duration(&self) -> Time {
        self.duration
    }
}

impl TimeRangeEditableTrait for TimeRange {
    fn set_start(&mut self, start: Time) {
        self.start = start;
    }

    fn set_duration(&mut self, duration: Time) {
        self.duration = duration;
    }
}
