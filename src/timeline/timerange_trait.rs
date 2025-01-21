use crate::core::Time;

/**
Defines basic functions for a TimeRange.

The default implement assumes the struct stores only the start time point
and the duration, the end time point will be calculated from the two `Time`.
*/
pub trait TimeRangeTrait {
    /// Start time of the TimeRange
    fn start(&self) -> Time;
    
    /// Duration of the TimeRange
    fn duration(&self) -> Time;

    /**
    End time of the TimeRange.
    
    Default implement depends on the result of `start()` and `duration()`.
    */
    fn end(&self) -> Time {
        self.start() + self.duration()
    }

    ///If this TimeRange contains a time point.
    fn contains(&self, time: &Time) -> bool {
        self.start() <= *time && *time <= self.end()
    }

    ///Check if this TimeRange is overlapped with another TimeRange.
    fn overlaps(&self, other: &dyn TimeRangeTrait) -> bool {
        // self.contains(&other.start()) || self.contains(&other.end()) || other.contains(&self.start()) || other.contains(&self.end())
        self.start() <= other.end() && self.end() >= other.start()
    }
}

/**
Mutable functions for TimeRange.

Provides functions to manipulate TimeRange.

Depends on TimeRangeTrait.
*/
pub trait TimeRangeEditableTrait
where
    Self: TimeRangeTrait,
{
    ///Set a new start time
    fn set_start(&mut self, start: Time);
    
    ///Set the duration fo the TimeRange
    fn set_duration(&mut self, duration: Time);
    
    /**
    Set the end time of the TimeRange.
    
    By default, it will set the duration of the TimeRange.
    */
    fn set_end(&mut self, end: Time) {
        self.set_duration(end - self.start());
    }
    
    /**
    Shift the time points of the TimeRange, duration remains.
    
    By default, it only shifts the start time point,
    Since the end point is always calculated from duration.
    */
    fn shift_time(&mut self, shift: Time) {
        self.set_start(self.start() + shift);
    }
}

impl PartialEq for dyn TimeRangeTrait {
    fn eq(&self, other: &Self) -> bool {
        self.start() == other.start() && self.duration() == other.duration()
    }
}

impl Eq for dyn TimeRangeTrait {}

impl PartialOrd for dyn TimeRangeTrait {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.start().partial_cmp(&other.start())
    }
}

impl Ord for dyn TimeRangeTrait {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start().cmp(&other.start())
    }
}
