#![allow(dead_code)]

use super::timebase::Timebase;
use super::timecode_support::*;
use std::hash::Hash;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug)]
pub struct Time {
    data: i128,
}

impl Time {
    pub fn new() -> Time {
        Time { data: 0 }
    }

    pub fn from_millisecond(m: i128) -> Time {
        Time { data: m }
    }

    pub fn from_seconds(seconds: &f64) -> Self {
        Time {
            data: (seconds * 1000.0).round() as i128,
        }
    }

    pub fn from_timecode(timecode: &str, timebase: &Timebase) -> Result<Self, TimecodeFormatError> {
        let parts = TimecodeParts::from_timecode(timecode)?;
        let mut ms = parts.hh as i128 * 60 * 60 * 1000;
        ms += parts.mm as i128 * 60 * 1000;
        ms += parts.ss as i128 * 1000;
        ms += parts.ff as i128 * timebase.milliseconds_per_frame() as i128;
        Ok(Time { data: ms })
    }
    
    pub fn to_timecode(&self, timebase: &Timebase) -> String {
        todo!()
    }

    pub fn from_timestamp(timecode: &str) -> Result<Self, TimecodeFormatError> {
        let parts = TimecodeParts::from_timestamp(timecode)?;
        let mut ms = parts.hh as i128 * 60 * 60 * 1000;
        ms += parts.mm as i128 * 60 * 1000;
        ms += parts.ss as i128 * 1000;
        ms += (parts.ff as f64 / 1000.0) as i128;
        Ok(Time { data: ms })
    }
    
    pub fn to_timestamp(&self) -> String{
        todo!()
    }
}

impl From<i128> for Time {
    fn from(data: i128) -> Time {
        Time { data }
    }
}

impl Clone for Time {
    fn clone(&self) -> Time {
        Time { data: self.data }
    }
}
impl Copy for Time {}

impl PartialEq for Time {
    fn eq(&self, other: &Time) -> bool {
        self.data == other.data
    }
}
impl Eq for Time {}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Time) -> Option<std::cmp::Ordering> {
        self.data.partial_cmp(&other.data)
    }
}
impl Ord for Time {
    fn cmp(&self, other: &Time) -> std::cmp::Ordering {
        self.data.cmp(&other.data)
    }
}

impl Hash for Time {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.data.hash(state);
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


