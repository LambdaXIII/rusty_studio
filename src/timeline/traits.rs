use std::any::Any;

pub trait ContentSupport {
    fn get_content<T>(&self) -> Option<T>
    where
        T: Any + Sync + Send + Clone;

    fn set_content<T>(&mut self, content: T)
    where
        T: Any + Sync + Send + Clone;

    fn clear_content(&mut self);
}


use crate::core::Time;


/**
TimeRange 设定了基本的对于时间段的支持

TimeRange 的默认实现要求对象保存开始时间点和时长两个信息，
结束时间点将根据这两个部分自动计算。
如果使用其它的方法保存时间信息，有可能需要重写全部三个方法。
其它的基于时间的方法也会根据这三个函数的返回值进行计算。

timeline模块中的很多内容都实现了或要求对象实现这个trait。
*/
pub trait TimeRange {
    fn start(&self) -> Time;
    fn duration(&self) -> Time;

    ///返回时间段的结束时间点。默认实现是根据 start 和 duration 计算的。
    fn end(&self) -> Time {
        self.start() + self.duration()
    }

    ///判断此时间段中是否包含某个时间点。
    fn contains(&self, time: &Time) -> bool {
        self.start() <= *time && *time <= self.end()
    }

    ///判断是否和另一个TimeRange相交。
    fn overlaps(&self, other: &dyn TimeRange) -> bool {
        // self.contains(&other.start()) || self.contains(&other.end()) || other.contains(&self.start()) || other.contains(&self.end())
        self.start() <= other.end() && self.end() >= other.start()
    }
}

pub trait TimeRangeEditable
where
    Self: TimeRange,
{
    fn set_start(&mut self, start: Time);
    fn set_duration(&mut self, duration: Time);
    fn set_end(&mut self, end: Time) {
        self.set_duration(end - self.start());
    }
    fn shift_time(&mut self, shift: Time) {
        self.set_start(self.start() + shift);
    }
}
