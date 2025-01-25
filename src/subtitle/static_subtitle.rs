#![allow(dead_code)]
use crate::core::Time;

pub struct StaticSubtitle {
    pub start: Time,
    pub duration: Time,
    pub content: String,
}
