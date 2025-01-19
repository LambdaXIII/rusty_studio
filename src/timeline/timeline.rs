use super::{TimeRange, Track, TrackManager};
use crate::core::MetadataSupport;
use std::collections::HashMap;

pub struct Timeline {
    tracks: Vec<Box<Track>>,
    metadata: Box<HashMap<String, String>>,
}

impl Timeline {
    pub fn new() -> Self {
        let mut tracks = Vec::new();
        tracks.push(Box::new(Track::default()));
        Self {
            tracks,
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
        let result = self.tracks.remove(index);
        if self.tracks.is_empty(){
            self.tracks.push(Box::new(Track::default()));
        }
        Some(result)
    }

    fn track_count(&self) -> usize {
        self.tracks.len()
    }

    fn clear_tracks(&mut self) {
        self.tracks.clear();
        self.tracks.push(Box::new(Track::default()))
    }

    fn auto_insert_item(&mut self, item: Box<dyn TimeRange>) {
        for i in 0..self.tracks.len(){
            let t = self.tracks.get_mut(i).unwrap();
            match t.try_add_item(item){
                Ok(_)=> return,
                Err(_) => continue,
            }
        }
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
