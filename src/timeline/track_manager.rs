use super::Track;

pub trait TrackManager {
    fn append_track(&mut self, track: Box<Track>);
    fn prepend_track(&mut self, track: Box<Track>);
    fn insert_track(&mut self, index: usize, track: Box<Track>);
    fn track_at(&self, index: usize) -> Option<&Box<Track>>;
    fn take_at(&mut self, index: usize) -> Option<Box<Track>>;
    fn track_count(&self) -> usize;
}
