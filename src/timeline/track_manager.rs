use super::{TimeRange,Track};
use crate::core::Time;


pub trait TrackManager<T:TimelineItem<T>+TimeRange> {
    type TrackType = Track<T>;
    fn new() -> Self;
    fn append_track(&mut self, track: Self::TrackType);
    fn prepend_track(&mut self, track: Self::TrackType);
    fn insert_track(&mut self, index: usize, track: Self::TrackType);
    fn track_at(&self, index: usize) -> Option<Box<Self::TrackType>>;
    fn remove_track(&mut self, index: usize);
    fn track_count(&self) -> usize;
}