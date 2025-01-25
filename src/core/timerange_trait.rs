use crate::core::Time;

/**
定义了基础的方法用于描述一个时间段。
Defines basic functions for a TimeRange.

默认的实现假定结构体只保存了开始时间点和时间段的时长。
结束时间点则通过前两个时间点来计算。

The default implement assumes the struct stores only the start time point
and the duration, the end time point will be calculated from the two `Time`.
*/
pub trait TimeRangeSupport {
    ///时间段的起始时间点 | Start time of the TimeRange
    fn start(&self) -> Time;
    
    ///时间段的时长 | Duration of the TimeRange
    fn duration(&self) -> Time;

    /**
    时间段的结束时间点 | End time of the TimeRange.
    
    默认实现从开始时间和时长计算得来。
    Default implement depends on the result of `start()` and `duration()`.
    */
    fn end(&self) -> Time {
        self.start() + self.duration()
    }

    ///检测时间段是否包含给定的时间点 | If this TimeRange contains a time point.
    fn contains(&self, time: &Time) -> bool {
        self.start() <= *time && *time <= self.end()
    }

    ///检查时间段是否和另一个时间段相交 | Check if this TimeRange is overlapped with another TimeRange.
    fn overlaps(&self, other: &dyn TimeRangeSupport) -> bool {
        // self.contains(&other.start()) || self.contains(&other.end()) || other.contains(&self.start()) || other.contains(&self.end())
        self.start() <= other.end() && self.end() >= other.start()
    }
}

/**
用于描述可变的时间段。
Mutable functions for TimeRange.

提供了一系列用于编辑时间段信息的方法。

Provides functions to manipulate TimeRange.

要求实现 TimeRangeTrait。

Depends on TimeRangeTrait.
*/
pub trait TimeRangeEditingSupport
where
    Self: TimeRangeSupport,
{
    ///设置开始时间点 | Set a new start time
    fn set_start(&mut self, start: Time);
    
    ///设置时长 | Set the duration fo the TimeRange
    fn set_duration(&mut self, duration: Time);
    
    /**
    设置结束时间点 | Set the end time of the TimeRange.
    
    默认实现中它将计算并修改片段的时长。
    
    By default, it will set the duration of the TimeRange.
    */
    fn set_end(&mut self, end: Time) {
        self.set_duration(end - self.start());
    }
    
    /**
    将时间段整体平移 | Shift the time points of the TimeRange, duration remains.
    
    默认实现中它将只改变开始时间点并保持时长不变。
    
    By default, it only shifts the start time point,
    Since the end point is always calculated from duration.
    */
    fn shift_time(&mut self, shift: Time) {
        self.set_start(self.start() + shift);
    }
}

impl PartialEq for dyn TimeRangeSupport {
    fn eq(&self, other: &Self) -> bool {
        self.start() == other.start() && self.duration() == other.duration()
    }
}

impl Eq for dyn TimeRangeSupport {}

impl PartialOrd for dyn TimeRangeSupport {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.start().partial_cmp(&other.start())
    }
}

impl Ord for dyn TimeRangeSupport {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start().cmp(&other.start())
    }
}
