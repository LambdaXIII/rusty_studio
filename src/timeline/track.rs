#![allow(dead_code)]

use super::{TimeRange, TimelineItem};
use crate::core::Time;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrackNoSafeInsertionError;

impl std::fmt::Display for TrackNoSafeInsertionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "No safe insertion point found.")
    }
}
impl std::error::Error for TrackNoSafeInsertionError {}

pub struct Track<T: TimeRange + TimelineItem<T>> {
    pub name: String,
    pub description: String,
    items: Vec<Box<T>>,
}

impl<T: TimeRange + TimelineItem<T>> Track<T> {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            items: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn find_insert_point(&self, item: &Box<T>) -> usize {
        self.items.partition_point(|i| i.start() < item.start())
    }

    pub fn check_insert_point(&self, index: usize, item: &Box<T>) -> bool {
        let mut items: Vec<&Box<T>> = Vec::new();
        for _ in (index - 1)..=(index + 1) {
            let tmp_item = self.items.get(index);
            if tmp_item.is_some() {
                items.push(tmp_item.unwrap());
            }
        }
        for it in items {
            if it.overlaps(item.as_ref()) {
                return false;
            }
        }
        true
    }

    pub fn insert_item(&mut self, index: usize, item: Box<T>) {
        self.items.insert(index, item);
    }

    pub fn force_add_item(&mut self, item: Box<T>) {
        let index = self.find_insert_point(&item);
        self.insert_item(index, item);
    }

    pub fn try_add_item(&mut self, item: Box<T>) -> Result<(), TrackNoSafeInsertionError> {
        let index = self.find_insert_point(&item);
        if self.check_insert_point(index, &item) {
            self.insert_item(index, item);
            return Ok(());
        }
        Err(TrackNoSafeInsertionError)
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn sort_items(&mut self) {
        self.items.sort_by(|a, b| a.start().cmp(&b.start()));
    }

    pub fn items(&self) -> &Vec<Box<T>> {
        &self.items
    }
}

impl<T: TimeRange + TimelineItem<T>> TimeRange for Track<T> {
    fn start(&self) -> Time {
        Time::new()
    }

    fn duration(&self) -> Time {
        if self.items.is_empty() {
            return Time::new();
        }
        self.items.last().unwrap().end()
    }
}
