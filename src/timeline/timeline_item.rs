#![allow(dead_code)]

use super::TimeRange;
use crate::core::{MetadataSupport, Time};
use std::collections::HashMap;

struct TimelineItem<T: Clone> {
    start: Time,
    duration: Time,
    content: Option<T>,
    metadata: Box<HashMap<String, String>>,
}

impl<T: Clone> TimelineItem<T> {
    fn new(start: Time, duration: Time, content: &T) -> Self {
        Self {
            start,
            duration,
            content: Some(content.clone()),
            metadata: Box::new(Default::default()),
        }
    }

    fn from_time_range(other: &dyn TimeRange) -> Self {
        Self {
            start: other.start(),
            duration: other.duration(),
            content: None,
            metadata: Box::new(Default::default()),
        }
    }

    pub fn content(&self) -> Option<T> {
        self.content.clone()
    }

    pub fn set_content(&mut self, value: &T) {
        self.content = Some(value.clone())
    }

    pub fn clear_content(&mut self) {
        self.content = None
    }

    pub fn set_start(&mut self, t: Time) {
        self.start = t
    }

    pub fn set_duration(&mut self, t: Time) {
        self.duration = t
    }

    pub fn set_end(&mut self, t: &Time) {
        self.duration = *t - self.start
    }

    pub fn shift_time(&mut self, t: &Time) {
        self.start += *t
    }
}

impl<T: Clone> Default for TimelineItem<T> {
    fn default() -> Self {
        Self {
            start: Time::default(),
            duration: Time::default(),
            content: None,
            metadata: Box::new(Default::default()),
        }
    }
}

impl<T: Clone> TimeRange for TimelineItem<T> {
    fn start(&self) -> Time {
        self.start
    }

    fn duration(&self) -> Time {
        self.duration
    }
}

impl <T:Clone> MetadataSupport for TimelineItem<T>{
    fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key,value);
    }

    fn get_metadata(&self, key: &String) -> Option<&String> {
        self.metadata.get(key)
    }

    fn get_all_metadata(&self) -> Box<HashMap<String, String>> {
        self.metadata.clone()
    }
}
