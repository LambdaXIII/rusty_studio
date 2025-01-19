use super::{Track, TrackManager};
use crate::core::MetadataSupport;
use std::collections::HashMap;

pub struct Timeline {
    tracks: Vec<Box<Track>>,
    metadata: Box<HashMap<String, String>>,
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            tracks: Vec::new(),
            metadata: Box::new(Default::default()),
        }
    }
}


impl TrackManager for Timeline {
    fn append_track(&mut self, track: Box<Track>) {
        self.tracks.push(track)
    }

    fn prepend_track(&mut self, track: Box<Track>) {
        self.tracks.insert(0, track)
    }

    fn insert_track(&mut self, index: usize, track: Box<Track>) {
        self.tracks.insert(index, track)
    }

    fn track_at(&self, index: usize) -> Option<&Box<Track>> {
        self.tracks.get(index)
    }

    fn take_at(&mut self, index: usize) -> Option<Box<Track>> {
        if index >= self.tracks.len() {
            return None;
        }
        Some(self.tracks.remove(index))
    }

    fn track_count(&self) -> usize {
        self.tracks.len()
    }
}

impl MetadataSupport for Timeline {
    fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    fn get_metadata(&self, key: &String) -> Option<&String> {
        self.metadata.get(key)
    }

    fn get_all_metadata(&self) -> Box<HashMap<String, String>> {
        self.metadata.to_owned()
    }
}
