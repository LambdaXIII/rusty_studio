#![allow(dead_code)]
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use super::{Item, Track, TimeRangeTrait, TimeRangeEditableTrait};
use crate::core::{DataBox,MetadataSupport};


pub struct Timeline{
    tracks:Vec<Rc<RefCell<Track>>>,
    metadata:RefCell<DataBox>,
}


impl Default for Timeline {
    fn default() -> Self {
        Self {
            tracks:vec![Rc::new(RefCell::new(Track::default()))],
            metadata:RefCell::new(DataBox::default()),
        }
    }
}

impl Clone for Timeline {
    fn clone(&self) -> Self {
        let mut res = Self::default();
        res.metadata = RefCell::new(self.metadata.borrow().clone());
        for t in &self.tracks{
            res.tracks.push(t.to_owned());
        }
        res
    }
}

impl Timeline {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn tracks_count(&self) -> usize {
        self.tracks.len()
    }
    
    pub fn clear_tracks(&mut self) {
        self.tracks.clear();
        self.tracks.push(Rc::new(RefCell::new(Track::default())));
    }
    
    pub fn push_track(&mut self, track: Track) {
        if self.tracks.len() == 1 && self.tracks[0].borrow().is_empty() {
            self.tracks.clear();
        }
        self.tracks.push(Rc::new(RefCell::new(track)));
    }
    
    pub fn take_track(&mut self, index: usize) -> Option<Rc<RefCell<Track>>> {
        if index >= self.tracks.len() {
            return None;
        }
        let result = Some(self.tracks.remove(index));
        if self.tracks.is_empty(){
            self.tracks.push(Rc::new(RefCell::new(Track::default())));
        }
        result
    }
    
    pub fn get_track(&self, index: usize) -> Option<Rc<RefCell<Track>>> {
        if index >= self.tracks.len() {
            return None;
        }
        Some(self.tracks[index].to_owned())
    }
    
    pub fn add_item(&mut self, item: Box<Item>) {
        let mut inserted = false;
        for i in 0..self.tracks.len() {
            let mut track = self.tracks[i].borrow_mut();
            match track.try_add_item(&item) {
                Ok(_) => {inserted = true;break;},
                Err(_) => continue,
            }
        }
        if !inserted {
            let mut track = Track::default();
            track.force_push_item(item);
            self.push_track(track);
        }
    }
}