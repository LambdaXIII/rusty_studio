use std::ops::Deref;
use crate::core::Time;
use super::{TimeRangeTrait, TimeRangeEditableTrait};

#[derive(Debug,Clone,Copy)]
pub struct TimeRange {
    start:Time,duration:Time
}

impl TimeRange {
    pub fn new(start:Time,duration:Time)->Self{
        Self{start,duration}
    }
    
    pub fn from_timerange(range:&dyn TimeRangeTrait) ->Self{
        Self{
            start:range.start(),
            duration:range.duration()
        }
    }
    
    pub fn whole_timerange<'a, I>(ranges:I) -> Self
    where I:IntoIterator<Item=&'a dyn TimeRangeTrait>
    {
        let mut start = Time::default();
        let mut end = Time::default();
        for range in ranges{
            if range.start() < start{
                start = range.start();
            }
            if range.end() > end{
                end = range.end();
            }
        }
        Self{
            start,
            duration:end-start
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
