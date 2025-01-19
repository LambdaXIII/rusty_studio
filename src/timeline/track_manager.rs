use super::{TimeRange, Track};


/**
TrackManager 定义了一系列操作多个 Track 的接口。

一般情况下不需要自己实现这些东西， Timeline结构体已经实现好了。
*/
pub trait TrackManager {
    fn append_track(&mut self, track: Box<Track>);
    fn prepend_track(&mut self, track: Box<Track>);
    fn insert_track(&mut self, index: usize, track: Box<Track>);
    fn track_at(&self, index: usize) -> Option<&Box<Track>>;
    fn take_at(&mut self, index: usize) -> Option<Box<Track>>;
    fn track_count(&self) -> usize;
    fn clear_tracks(&mut self);
    fn auto_insert_item(&mut self,item: Box<dyn TimeRange>);
}
