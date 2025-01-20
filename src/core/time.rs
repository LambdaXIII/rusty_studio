#![allow(dead_code)]

use super::timebase::Timebase;
use super::timecode_parts::*;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};


/**
Time 用于表示某一时刻的时间或一段时间。

因为大部分的多媒体制作中，时间都是以毫秒为单位的，所以 Time 默认的时间精度也**精确到毫秒**。

Time 是一个不可变类型，所以你不能直接修改 Time 的值。
Time 可以通过 `Time::from_millisecond()` 或 `Time::from_seconds()` 来创建一个新的 Time。
而 `Time::new()` 或 `Time::default()` 会返回一个新的 Time，其值为 0。

Time 可以依据时间大小排序，也可以互相进行加减运算；也可以乘以或者除以一个数字以达到计算数倍时间的效果。
*注意 Time 不可以除以 0.*

另外，Time 也提供了对于时间码文本的支持。
Time 可以通过 `Time::from_timecode()` 或 `Time::from_timestamp()` 来创建一个新的 Time。
也可以通过 `Time::to_timecode()` 或 `Time::to_timestamp()` 来将 Time 转换为时间码文本。
其中形如 `hh:mm:ss:ff` 的形式被称为 `时间码`，时间码在转换时需要通过 `Timebase` 提供时基信息；
而形如 `hh:mm:ss:MMM` 的形式被称为 `时间戳`，其中的 `MMM` 为毫秒数，所以时间戳不需要时基信息。

-----

Time is a structure that represents a time or a duration.

Because most of the multimedia making uses milliseconds as the unit,
the default precision of Time is also **exact to milliseconds**.

Time is an immutable type, so you cannot directly modify the value of Time.
Time can be created by `Time::from_millisecond()` or `Time::from_seconds()`.
Time can be sorted by time itself, or can be added or subtracted.
Time can also be multiplied or divided by a number to achieve the effect of calculating the number of times.
Of course, ** Time cannot be divided by 0. **

Time also provides support for timecode text.
Time can be created by `Time::from_timecode()` or `Time::from_timestamp()`.
Time can also be converted to timecode text by `Time::to_timecode()` or `Time::to_timestamp()`.
The form of `hh:mm:ss:ff` is called `timecode`, and the timecode needs to provide `Timebase` information when converting;
The form of `hh:mm:ss:MMM` is called `timestamp`, where `MMM` is milliseconds, so timestamp does not need timebase information.
*/
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Time {
    data: i128,
}

impl Default for Time {
    fn default() -> Self {
        Time { data: 0 }
    }
}

impl Time {
    pub fn new(m: i128) -> Time {
        Time { data: m }
    }

    pub fn from_millisecond(m: i128) -> Time {
        Time { data: m }
    }

    pub fn to_millisecond(&self) -> i128 {
        self.data
    }

    pub fn from_seconds(seconds: &f64) -> Self {
        Time {
            data: (seconds * 1000.0).round() as i128,
        }
    }

    /**
    从时间码文本创建一个新的 Time。
    时间码文本使用正则表达式判断并解析，如果解析失败，将会返回一个 `TimecodeFormatError` 错误。
    
    注意：`时间码` 在本工具集中特指 `hh:mm:ss:ff` 的形式。
    -----
    Create a new Time from timecode text.
    The timecode text is parsed using a regular expression and checked.
    Note: `timecode` refers to the form of `hh:mm:ss:ff` in this toolset.
    */
    pub fn from_timecode(timecode: &str, timebase: &Timebase) -> Result<Self, TimecodeFormatError> {
        let parts = TimecodeParts::from_timecode(timecode)?;
        let mut ms = parts.hh as i128 * 60 * 60 * 1000;
        ms += parts.mm as i128 * 60 * 1000;
        ms += parts.ss as i128 * 1000;
        ms += parts.ff as i128 * timebase.milliseconds_per_frame() as i128;
        Ok(Time { data: ms })
    }

    /**
    将 Time 转换为时间码文本。
    其作用和 `Time::from_timecode()` 相反。
    */
    pub fn to_timecode(&self, timebase: &Timebase) -> String {
        let milliseconds = (self.data % 1000) as f64 / 1000.0;
        let ff = (milliseconds / 1000.0 * timebase.fps as f64) as u32;
        let seconds = self.data / 1000;
        let ss = (seconds % 60) as u8;
        let minutes = seconds / 60;
        let mm = (minutes % 60) as u8;
        let hours = minutes / 60;
        let hh = (hours % 24) as u8;
        TimecodeParts {
            hh,
            mm,
            ss,
            ff,
            drop_frame: timebase.drop_frame,
        }
        .to_timecode()
    }

    /**
    从时间戳文本创建一个新的 Time。
    时间戳文本使用正则表达式判断并解析，如果解析失败，将会返回一个 `TimecodeFormatError` 错误。

    注意：`时间戳` 在本工具集中特指 `hh:mm:ss:MMM` 的形式。
    ---
    Create a new Time from timestamp text.
    The timestamp text is parsed using a regular expression and checked.
    
    Note: `timestamp` refers to the form of `hh:mm:ss:MMM` in this toolset.
    */
    pub fn from_timestamp(timecode: &str) -> Result<Self, TimecodeFormatError> {
        let parts = TimecodeParts::from_timestamp(timecode)?;
        let mut ms = parts.hh as i128 * 60 * 60 * 1000;
        ms += parts.mm as i128 * 60 * 1000;
        ms += parts.ss as i128 * 1000;
        ms += (parts.ff as f64 / 1000.0) as i128;
        Ok(Time { data: ms })
    }

    pub fn to_timestamp(&self) -> String {
        let ff = (self.data % 1000) as u32;
        let seconds = self.data / 1000;
        let ss = (seconds % 60) as u8;
        let minutes = seconds / 60;
        let mm = (minutes % 60) as u8;
        let hours = minutes / 60;
        let hh = (hours % 24) as u8;
        TimecodeParts {
            hh,
            mm,
            ss,
            ff,
            drop_frame: false,
        }
        .to_timestamp()
    }
}

impl From<i128> for Time {
    fn from(data: i128) -> Time {
        Time { data }
    }
}

impl Add<Time> for Time {
    type Output = Time;
    fn add(self, other: Time) -> Time {
        Time {
            data: self.data + other.data,
        }
    }
}

impl Sub<Time> for Time {
    type Output = Time;
    fn sub(self, other: Time) -> Time {
        Time {
            data: self.data - other.data,
        }
    }
}

impl Mul<f64> for Time {
    type Output = Time;
    fn mul(self, other: f64) -> Time {
        let m = self.data as f64 * other;
        let data = m.round() as i128;
        Time { data }
    }
}

impl Div<f64> for Time {
    type Output = Time;
    fn div(self, other: f64) -> Time {
        let m = self.data as f64 / other;
        let data = m.round() as i128;
        Time { data }
    }
}

impl AddAssign<Time> for Time {
    fn add_assign(&mut self, rhs: Time) {
        self.data += rhs.data;
    }
}

impl SubAssign<Time> for Time {
    fn sub_assign(&mut self, rhs: Time) {
        self.data -= rhs.data;
    }
}
