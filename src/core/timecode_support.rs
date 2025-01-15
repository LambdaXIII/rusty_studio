#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimecodeFormatError;

impl std::fmt::Display for TimecodeFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid Timecode Format")
    }
}

impl std::error::Error for TimecodeFormatError {}

use regex::Regex;

pub struct TimecodeParts {
    pub hh: u8,
    pub mm: u8,
    pub ss: u8,
    pub ff: u32,
    pub drop_frame: bool,
}

impl TimecodeParts {
    pub fn from_timecode(tc: &str) -> Result<Self, TimecodeFormatError> {
        let re = Regex::new(r"(\d{2}):(\d{2}):(\d{2})([;:])(\d{2})").unwrap();

        if !re.is_match(tc) {
            return Err(TimecodeFormatError);
        }

        let captures = re.captures(tc).unwrap();

        let hours: u8 = captures[1].parse().map_err(|_| TimecodeFormatError)?;
        let minutes: u8 = captures[2].parse().map_err(|_| TimecodeFormatError)?;
        let sep: String = captures[3].parse().map_err(|_| TimecodeFormatError)?;
        let seconds: u8 = captures[4].parse().map_err(|_| TimecodeFormatError)?;
        let frames: u32 = captures[5].parse().map_err(|_| TimecodeFormatError)?;

        Ok(TimecodeParts {
            hh: hours,
            mm: minutes,
            ss: seconds,
            ff: frames,
            drop_frame: sep == ";",
        })
    }

    pub fn from_timestamp(tc: &str) -> Result<Self, TimecodeFormatError> {
        let re = Regex::new(r"(\d{2}):(\d{2}):(\d{2})[.,:;](\d{3})").unwrap();

        if !re.is_match(tc) {
            return Err(TimecodeFormatError);
        }

        let captures = re.captures(tc).unwrap();

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

    pub fn to_timecode(&self) -> String {
        let sep = if self.drop_frame { ";" } else { ":" };
        format!(
            "{:02}:{:02}:{:02}{}{:02}",
            self.hh, self.mm, self.ss, sep, self.ff
        )
    }

    pub fn to_timestamp(&self) -> String {
        format!(
            "{:02}:{:02}:{:02}.{:03}",
            self.hh, self.mm, self.ss, self.ff
        )
    }
}
