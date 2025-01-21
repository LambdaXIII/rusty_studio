#![allow(dead_code)]

///在时间码字符串解析出错时抛出的错误。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimecodeFormatError;

impl std::fmt::Display for TimecodeFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid Timecode Format")
    }
}

impl std::error::Error for TimecodeFormatError {}

use regex::Regex;


/**
简单地保存时间码的各个部分，并将他们排版成为时间码或时间戳。

通常你也许并不需要用到这个结构体，因为它只是从 `Time` 的相关功能中分离出来而已。
但是如果需要的话，你也可以使用它作为一个工具来实现自己的时间码生成功能。

TimecodeParts simply stores all the parts of a timecode,
and struct a timecode/timestamp string from them.

Usually, you don't need to use it, since it is just a separated part of `Time`.
But, you still can use it to construct timecode/timestamp strings in your own struct.
*/
pub struct TimecodeParts {
    pub hh: u8,
    pub mm: u8,
    pub ss: u8,
    pub ff: u32,
    pub drop_frame: bool,
}

impl TimecodeParts {
    /**
    Parse timecode parts from a String.
    
    Example:
    ```rust
    # use rusty_studio::core::TimecodeParts;
    let parts = TimecodeParts::from_timecode("00:00:05:15").unwrap();
    assert_eq!(parts.hh, 0);
    assert_eq!(parts.mm, 0);
    assert_eq!(parts.ss, 5);
    assert_eq!(parts.ff, 15);
    assert_eq!(parts.drop_frame, false);
    ```
    
    ```rust
    # use rusty_studio::core::TimecodeParts;
    let parts = TimecodeParts::from_timecode("wrong");
    assert!(parts.is_err());
    ```
    
    ```rust
    # use rusty_studio::core::TimecodeParts;
    let parts = TimecodeParts::from_timecode("00:00:05;15").unwrap();
    assert_eq!(parts.drop_frame,true)
    ```
    */
    pub fn from_timecode(tc: &str) -> Result<Self, TimecodeFormatError> {
        let re = Regex::new(r"(\d{2}):(\d{2}):(\d{2})([;:])(\d{2})").unwrap();

        let captures = re.captures(tc);
        if captures.is_none() {
            return Err(TimecodeFormatError);
        }

        let captures = captures.unwrap();

        let hours: u8 = captures[1].parse().map_err(|_| TimecodeFormatError)?;
        let minutes: u8 = captures[2].parse().map_err(|_| TimecodeFormatError)?;
        let seconds: u8 = captures[3].parse().map_err(|_| TimecodeFormatError)?;
        let sep: String = captures[4].parse().map_err(|_| TimecodeFormatError)?;
        let frames: u32 = captures[5].parse().map_err(|_| TimecodeFormatError)?;

        Ok(TimecodeParts {
            hh: hours,
            mm: minutes,
            ss: seconds,
            ff: frames,
            drop_frame: sep == ";",
        })
    }

    /**
    Parse timestamp parts from a String.
    
    Example:
    ```rust
    # use rusty_studio::core::TimecodeParts;
    let parts = TimecodeParts::from_timestamp("12:34:56.789").unwrap();
    assert_eq!(parts.hh, 12);
    assert_eq!(parts.mm, 34);
    assert_eq!(parts.ss, 56);
    assert_eq!(parts.ff, 789);
    assert_eq!(parts.drop_frame, false);
    ```
    
    ```rust
    # use rusty_studio::core::TimecodeParts;
    let parts = TimecodeParts::from_timestamp("wrong");
    assert!(parts.is_err());
    ```
    */
    pub fn from_timestamp(tc: &str) -> Result<Self, TimecodeFormatError> {
        let re = Regex::new(r"(\d{2}):(\d{2}):(\d{2})[.,:;](\d{3})").unwrap();

        let captures = re.captures(tc);
        if captures.is_none() {
            return Err(TimecodeFormatError);
        }

        let captures = captures.unwrap();

        let hours: u8 = captures[1].parse().map_err(|_| TimecodeFormatError)?;
        let minutes: u8 = captures[2].parse().map_err(|_| TimecodeFormatError)?;
        let seconds: u8 = captures[3].parse().map_err(|_| TimecodeFormatError)?;
        let frames: u32 = captures[4].parse().map_err(|_| TimecodeFormatError)?;

        Ok(TimecodeParts {
            hh: hours,
            mm: minutes,
            ss: seconds,
            ff: frames,
            drop_frame: false,
        })
    }

    /**
    Construct a timecode String.
    
    Example:
    ```rust
    # use rusty_studio::core::TimecodeParts;
    let parts = TimecodeParts{
        hh:12,
        mm:34,
        ss:56,
        ff:78,
        drop_frame:false,
    };
    let timecode = parts.to_timecode();
    assert_eq!(timecode,"12:34:56:78");
    ```
    
    ```rust
    # use rusty_studio::core::TimecodeParts;
    let parts = TimecodeParts{
        hh:1,
        mm:2,
        ss:3,
        ff:45,
        drop_frame:true,
    };
    let timecode = parts.to_timecode();
    assert_eq!(timecode,"01:02:03;45");
    ```
    */
    pub fn to_timecode(&self) -> String {
        let sep = if self.drop_frame { ";" } else { ":" };
        format!(
            "{:02}:{:02}:{:02}{}{:02}",
            self.hh, self.mm, self.ss, sep, self.ff
        )
    }

    /**
    Construct a timestamp String.

    Example:
    ```rust
    # use rusty_studio::core::TimecodeParts;
    let parts = TimecodeParts{
        hh:12,
        mm:34,
        ss:56,
        ff:789,
        drop_frame:false,
    };
    let ts = parts.to_timestamp();
    assert_eq!(ts,"12:34:56.789");
    ```
    */
    pub fn to_timestamp(&self) -> String {
        format!(
            "{:02}:{:02}:{:02}.{:03}",
            self.hh, self.mm, self.ss, self.ff
        )
    }
}
