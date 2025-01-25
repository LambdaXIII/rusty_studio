#![allow(dead_code)]

use super::{Item, Track};
use crate::core::{DataBox, MetadataSupport};
use std::any::Any;

/**
基本的时间线。
Basic timeline with track management.

```rust
# use rusty_studio::timeline::{Item, TimeRange, TimeRangeTrait, Timeline, Track};
let mut a_timeline = Timeline::default();
assert_eq!(a_timeline.tracks_count(),1); // Timeline 默认情况下会有一个空轨道

fn mk_item(s:i128,d:i128,c:&str) -> Box<Item>{
    Box::new(Item::new(s,d,String::from(c)))
}

a_timeline.add_item(mk_item(0,10,"clip1"));
a_timeline.add_item(mk_item(50,100,"clip2"));
a_timeline.add_item(mk_item(20,10,"clip3"));
// add_item 会自动插入片段

assert_eq!(a_timeline.tracks_count(),1); // 这三个片段不交叉，所以都会插入在默认新建的轨道中

// 三个片段将会按照时间顺序排列
let track:&Box<Track> = a_timeline.get_track(0).unwrap();
assert_eq!(track.get(0).unwrap().get_content::<String>().unwrap(),String::from("clip1"));
assert_eq!(track.get(1).unwrap().get_content::<String>().unwrap(),String::from("clip3"));
assert_eq!(track.get(2).unwrap().get_content::<String>().unwrap(),String::from("clip2"));

// 如果没有合适的空间插入片段，将会自动新建轨道并插入
a_timeline.add_item(mk_item(5,20,"clip4")); //新建轨道
a_timeline.add_item(mk_item(45,30,"clip5"));//刚刚新建的轨道可用
a_timeline.add_item(mk_item(20,10,"clip6"));//再次新建轨道

assert_eq!(a_timeline.tracks_count(),3); // 这三个片段不交叉，所以都会插入在默认新建的轨道中

let track:&Box<Track> = a_timeline.get_track(1).unwrap();
assert_eq!(track.get(0).unwrap().get_content::<String>().unwrap(),String::from("clip4"));
assert_eq!(track.get(1).unwrap().get_content::<String>().unwrap(),String::from("clip5"));
let track:&Box<Track> = a_timeline.get_track(2).unwrap();
assert_eq!(track.get(0).unwrap().get_content::<String>().unwrap(),String::from("clip6"));
```
*/
pub struct Timeline {
    tracks: Vec<Box<Track>>,
    metadata: DataBox,
}

impl Default for Timeline {
    fn default() -> Self {
        Self {
            tracks: vec![Box::new(Track::default())],
            metadata: DataBox::default(),
        }
    }
}

impl Clone for Timeline {
    fn clone(&self) -> Self {
        let mut res = Self::default();
        res.metadata = self.metadata.clone();
        for t in &self.tracks {
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

    pub fn clear(&mut self) {
        self.tracks.clear();
        self.tracks.push(Box::new(Track::default()));
    }

    pub fn push_track(&mut self, track: Box<Track>) {
        let last = self.tracks.last();
        if last.is_none() && last.unwrap().is_empty() {
            self.tracks.pop();
        }
        self.tracks.push(track);
    }

    pub fn take_track(&mut self, index: usize) -> Option<Box<Track>> {
        if index >= self.tracks.len() {
            return None;
        }
        let result = Some(self.tracks.remove(index));
        if self.tracks.is_empty() {
            self.tracks.push(Box::new(Track::default()));
        }
        result
    }

    pub fn get_track(&self, index: usize) -> Option<&Box<Track>> {
        self.tracks.get(index)
    }

    pub fn add_item(&mut self, item: Box<Item>) {
        let mut inserted = false;
        for i in 0..self.tracks.len() {
            match self.tracks.get_mut(i).unwrap().try_add_item(&item) {
                Ok(_) => {
                    inserted = true;
                    break;
                }
                Err(_) => continue,
            }
        }
        if !inserted {
            let mut track = Box::new(Track::default());
            track.force_push_item(item);
            self.tracks.push(track);
        }
    }

    pub fn iter_tracks(&self) -> impl Iterator<Item=&Box<Track>> {
        self.tracks.iter()
    }
}

impl MetadataSupport for Timeline {
    fn get_metadata<T: Any + Send + Sync + Clone>(&self, key: &str) -> Option<T> {
        self.metadata.get(key)
    }

    fn set_metadata<T: Any + Send + Sync + Clone>(&mut self, key: &str, value: T) {
        self.metadata.set(key, value)
    }

    fn erase_metadata(&mut self, key: &String) {
        self.metadata.erase(key)
    }

    fn clear_metadata(&mut self) {
        self.metadata.clear()
    }
}
