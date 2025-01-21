#![allow(dead_code)]

use super::timebase::Timebase;
use super::timecode_parts::*;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

/**
Represents a time vector.

It can represent a moment or a duration, but its essence is a one-dimensional vector.
In other words, ** Time can be a negative value. **
So use it carefully to avoid any errors caused by the direction of time.
This toolkit is not responsible for any errors caused by the direction of time.

Because most of the multimedia making uses milliseconds as the unit,
the default precision of Time is also **exact to milliseconds**.

Time is an immutable type, so you cannot directly modify the value of Time.
Time can be created by `Time::from_millisecond()` or `Time::from_seconds()`.
Time can be sorted by time itself, or can be added or subtracted.
Time can also be multiplied or divided by a number to achieve the effect of calculating the number of times.
Of course, **Time cannot be divided by 0.**

Time also provides support for timecode text.
Time can be created by `Time::from_timecode()` or `Time::from_timestamp()`.
Time can also be converted to timecode text by `Time::to_timecode()` or `Time::to_timestamp()`.
The form of `hh:mm:ss:ff` is called `timecode`, and the timecode needs to provide `Timebase` information when converting;
The form of `hh:mm:ss.MMM` is called `timestamp`, where `MMM` is milliseconds, so timestamp does not need timebase information.

-----

表示一个时间向量。

它可以表示一个时刻或一段时长，但是本质上它是表示时间的一维向量。
也就是说 **Time 可以是一个负值** ，所以使用时请务必小心时间的方向，
本工具集不对时间方向错乱导致的任何灾难负责。

因为大部分的多媒体制作中，时间都是以毫秒为单位的，所以 Time 默认的时间精度也**精确到毫秒**。

Time 是一个不可变类型，所以你不能直接修改 Time 的值。
Time 可以通过 `Time::from_millisecond()` 或 `Time::from_seconds()` 来创建一个新的 Time。
而 `Time::new()` 或 `Time::default()` 会返回一个新的 Time，其值为 0。

Time 可以依据时间大小排序，也可以互相进行加减运算；也可以乘以或者除以一个数字以达到计算数倍时间的效果。
**注意 Time 不可以除以 0.**

另外，Time 也提供了对于时间码文本的支持。
Time 可以通过 `Time::from_timecode()` 或 `Time::from_timestamp()` 来创建一个新的 Time。
也可以通过 `Time::to_timecode()` 或 `Time::to_timestamp()` 来将 Time 转换为时间码文本。
其中形如 `hh:mm:ss:ff` 的形式被称为 `时间码`，时间码在转换时需要通过 `Timebase` 提供时基信息；
而形如 `hh:mm:ss.MMM` 的形式被称为 `时间戳`，其中的 `MMM` 为毫秒数，所以时间戳不需要时基信息。
*/
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Time {
    data: i128,
}

impl Default for Time {
    /**
    Construct a default Time, its value is 0.
    
    Example:
    ```rust
    # use rusty_studio::core::Time;
    let time = Time::default();
    assert_eq!(time.to_millisecond(), 0);
    ```
    */
    fn default() -> Self {
        Time { data: 0 }
    }
}

impl Time {
    ///直接通过一个 i128 毫秒数创建一个新的 Time。
    pub fn new(m: i128) -> Time {
        Time { data: m }
    }

    
    ///通过一个 i128 毫秒数创建一个新的 Time。
    pub fn from_millisecond(m: i128) -> Time {
        Time { data: m }
    }
    
    ///转换为毫秒数。其实是直接读取了内部的数据。
    pub fn to_millisecond(&self) -> i128 {
        self.data
    }

    ///转换为秒（作为浮点数）。
    pub fn to_second(&self) -> f64 {
        self.data as f64 / 1000.0
    }

    fn milliseconds_from_seconds(seconds: f64) -> i128 {
        (seconds * 1000.0).round() as i128
    }

    /**
    Construct Time from a f64 seconds.
    Value of seconds will be rounded to the nearest millisecond.

    Example:
    ```rust
    # use rusty_studio::core::Time;
    let time = Time::from_seconds(1.5);
    assert_eq!(time.to_millisecond(), 1500);
    let time = Time::from_seconds(1.23456);
    assert_eq!(time.to_millisecond(), 1235);
    ```
    */
    pub fn from_seconds(seconds: f64) -> Self {
        Time {
            data: Self::milliseconds_from_seconds(seconds),
        }
    }

    /**
    从时间码文本创建一个新的 Time。
    
    时间码文本使用正则表达式判断并解析，如果解析失败，将会返回一个 `TimecodeFormatError` 错误。

    注意：`时间码` 在本工具集中特指 `hh:mm:ss:ff` 的形式。
    
    Create a new Time from timecode text.
    The timecode text is parsed using a regular expression and checked.
    Note: `timecode` refers to the form of `hh:mm:ss:ff` in this toolset.

    Example:
    ```rust
    # use rusty_studio::core::{Time,Timebase,TimecodeFormatError};
    let time = Time::from_timecode("00:00:05:15", &Timebase{fps:30,drop_frame:false});
    assert_eq!(time.unwrap().to_millisecond(), 5500);
    let time = Time::from_timecode("00:00:10:30", &Timebase{fps:60,drop_frame:false});
    assert_eq!(time.unwrap().to_millisecond(), 10500);
    let time = Time::from_timecode("something wrong", &Timebase{fps:60,drop_frame:true});
    assert!(time.is_err());
    ```
    */
    pub fn from_timecode(timecode: &str, timebase: &Timebase) -> Result<Self, TimecodeFormatError> {
        let parts = TimecodeParts::from_timecode(timecode)?;
        let mut ms = parts.hh as i128 * 60 * 60 * 1000;
        ms += parts.mm as i128 * 60 * 1000;
        ms += parts.ss as i128 * 1000;
        ms += timebase.milliseconds_from_frames(parts.ff as u64);
        Ok(Time { data: ms })
    }

    /**
    将 Time 转换为时间码文本。
    
    其作用和 `Time::from_timecode()` 相反。

    Example:
    ```rust
    # use rusty_studio::core::{Time,Timebase};
    let time = Time::from_millisecond(5500);
    let timecode = time.to_timecode(&Timebase{fps:30,drop_frame:false});
    assert_eq!(timecode, "00:00:05:15");
    ```
    */
    pub fn to_timecode(&self, timebase: &Timebase) -> String {
        let ms = (self.data % 1000) as u32;
        let ff = timebase.frames_from_milliseconds(ms as i128) as u32;
        let seconds = self.to_second() as u64;
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
    
    Create a new Time from timestamp text.
    The timestamp text is parsed using a regular expression and checked.

    Note: `timestamp` refers to the form of `hh:mm:ss:MMM` in this toolset.
    
    Example:
    ```rust
    # use rusty_studio::core::{Time,TimecodeFormatError};
    let time = Time::from_timestamp("00:00:05.150");
    assert_eq!(time.unwrap().to_millisecond(), 5150);
    let time = Time::from_timestamp("00:00:10.300");
    assert_eq!(time.unwrap().to_millisecond(), 10300);
    let time = Time::from_timestamp("something wrong");
    assert!(time.is_err());
    ```
    */
    pub fn from_timestamp(timecode: &str) -> Result<Self, TimecodeFormatError> {
        let parts = TimecodeParts::from_timestamp(timecode)?;
        let mut ms = parts.hh as i128 * 60 * 60 * 1000;
        ms += parts.mm as i128 * 60 * 1000;
        ms += parts.ss as i128 * 1000;
        ms += parts.ff as i128;
        Ok(Time { data: ms })
    }

    /**
    将 Time 转换为时间戳文本。
    
    其作用和 `Time::from_timestamp()` 相反。
    Example:
    ```rust
    # use rusty_studio::core::Time;
    let time = Time::from_millisecond(5500);
    let timestamp = time.to_timestamp();
    assert_eq!(timestamp, "00:00:05.500");
    ```
    */
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

impl Into<i128> for Time{
    fn into(self) -> i128 {
        self.data
    }
}

/**
Time 可以和 Time 相加，

相加之后的 Time 为两个时间向量之和。

Example:
```rust
# use rusty_studio::core::Time;
let time1 = Time::from_millisecond(1000);
let time2 = Time::from_millisecond(2000);
let time3 = time1 + time2;
assert_eq!(time3.to_millisecond(), 3000);
```

```rust
# use rusty_studio::core::Time;
let time1 = Time::from_millisecond(1000);
let time2 = Time::from_millisecond(-2000);
let time3 = time1 + time2;
assert_eq!(time3.to_millisecond(), -1000);
```
*/
impl Add<Time> for Time {
    type Output = Time;
    fn add(self, other: Time) -> Time {
        Time {
            data: self.data + other.data,
        }
    }
}

/**
Time can also subtract another Time.

Example:
```rust
# use rusty_studio::core::Time;
let time1 = Time::from_millisecond(1000);
let time2 = Time::from_millisecond(2000);
let time3 = time1 - time2;
assert_eq!(time3.to_millisecond(), -1000);
```
*/
impl Sub<Time> for Time {
    type Output = Time;
    fn sub(self, other: Time) -> Time {
        Time {
            data: self.data - other.data,
        }
    }
}

/**
Time can also multiply or divide by a number.
Example:
```rust
# use rusty_studio::core::Time;
let time1 = Time::from_millisecond(1000);
let time2 = time1 * 2.0;
assert_eq!(time2.to_millisecond(), 2000);
```

```rust
# use rusty_studio::core::Time;
let time1 = Time::from_millisecond(1000);
let time2 = time1 / 2.0;
assert_eq!(time2.to_millisecond(), 500);
```
*/
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
    /// Provide operator `+=` to another Time
    fn add_assign(&mut self, rhs: Time) {
        self.data += rhs.data;
    }
}

impl SubAssign<Time> for Time {
    ///Provide operator `-=` to another Time
    fn sub_assign(&mut self, rhs: Time) {
        self.data -= rhs.data;
    }
}
