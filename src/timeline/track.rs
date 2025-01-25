#![allow(dead_code)]
use super::{Item};
use crate::core::{DataBox, MetadataSupport, Time, TimeRangeSupport};
use std::any::Any;
use std::cell::RefCell;
use std::ops::Deref;

pub struct Track {
    items: Vec<Box<Item>>,
    metadata: DataBox,
    end_cache: RefCell<Option<Time>>,
}

impl Default for Track {
    fn default() -> Self {
        Self {
            items: vec![],
            metadata: DataBox::default(),
            end_cache: RefCell::new(None),
        }
    }
}

impl Clone for Track {
    fn clone(&self) -> Self {
        Self {
            items: self.items.clone(),
            metadata: self.metadata.clone(),
            end_cache: RefCell::new(None),
        }
    }
}

impl Track {
    pub fn new() -> Self {
        Self::default()
    }

    fn update_end_cache(&mut self, item: &dyn TimeRangeSupport) {
        if self.end_cache.borrow().is_some() {
            let x = self.end_cache.borrow().unwrap();
            if x < item.end() {
                self.end_cache.borrow_mut().replace(item.end());
            }
        }
    }

    pub fn iter_items(&self) -> impl Iterator<Item = &Box<Item>> {
        self.items.iter()
    }

    pub fn force_push_item(&mut self, item: Box<Item>) {
        self.update_end_cache(item.deref());
        self.items.push(item);
    }

    pub fn force_insert_item(&mut self, index: usize, item: Box<Item>) -> Result<usize, usize> {
        let insert_point = {
            if self.items.is_empty() {
                0
            } else if index >= self.items.len() {
                self.items.len()
            } else {
                index
            }
        };
        self.update_end_cache(item.deref());
        self.items.insert(insert_point, item);
        if insert_point == index {
            Ok(insert_point)
        } else {
            Err(insert_point)
        }
    }

    pub fn force_sort_items(&mut self) {
        self.items.sort_by(|a, b| a.start().cmp(&b.start()));
        let new_end = self.items.last().and_then(|x| Some(x.end()));
        self.end_cache.replace(new_end);
    }

    /**
    根据时间段信息查找预备插入位置。
    Search for insert point for a time range.

    ```rust
    # use rusty_studio::timeline::{Item,TimeRange,Track};
    # use rusty_studio::core::TimeRangeSupport;
    let mut track = Track::default();
    let item = Item::from_timerange(TimeRange::from_millisecond(35,30));
    assert_eq!(track.find_insert_point(&item),0); //当不包含任何元素时，插入位置为0

    track.force_push_item(Box::new(Item::from_timerange(TimeRange::from_millisecond(5,5))));
    track.force_push_item(Box::new(Item::from_timerange(TimeRange::from_millisecond(30,5))));
    track.force_push_item(Box::new(Item::from_timerange(TimeRange::from_millisecond(100,5))));
    track.force_push_item(Box::new(Item::from_timerange(TimeRange::from_millisecond(200,5))));

    assert_eq!(track.find_insert_point(&item),2);
    ```
    */
    pub fn find_insert_point(&self, item: &dyn TimeRangeSupport) -> usize {
        if self.items.is_empty() {
            return 0;
        }
        if self.end_cache.borrow().is_some() && self.end_cache.borrow().unwrap() < item.start() {
            return self.items.len();
        }

        let search = self
            .items
            .binary_search_by(|x| x.start().cmp(&item.start()));
        search.unwrap_or_else(|index| index)
    }

    /**
    检查插入位置是否安全。
    Check if the insert point is safe.

    “安全的”插入位置指的是：**插入位置前后的元素与待插入的元素的时间段不相交** 。
    "Safe" insert point means: **The insert point before and after the element and the element to be inserted do not intersect in time.**

    ```rust
    # use rusty_studio::timeline::{Item,TimeRange,Track};
    # use rusty_studio::core::TimeRangeSupport;
    let mut track = Track::default();
    let item1 = Item::from_timerange(TimeRange::from_millisecond(35,30));
    let item2 = Item::from_timerange(TimeRange::from_millisecond(45,10));

    track.force_push_item(Box::new(Item::from_timerange(TimeRange::from_millisecond(5,5))));
    track.force_push_item(Box::new(Item::from_timerange(TimeRange::from_millisecond(30,10))));
    track.force_push_item(Box::new(Item::from_timerange(TimeRange::from_millisecond(100,10))));

    assert_eq!(track.check_insert_point(2,&item1),false); //插入位置2与item1相交
    assert_eq!(track.check_insert_point(2,&item2),true); //插入位置2与item2不相交
    ```
    */
    pub fn check_insert_point(&self, index: usize, item: &dyn TimeRangeSupport) -> bool {
        if index >= self.items.len() {
            return true;
        }
        if index == 0 {
            return item.end() <= self.items[index].start();
        }

        for i in index - 1..=index + 1 {
            let current = self.items.get(i);
            match current {
                None => continue,
                Some(current_item) => {
                    if current_item.overlaps(item) {
                        return false;
                    }
                }
            }
        }
        true
    }

    /**
    向轨道中（强制）添加一个片段。
    Add an item to the track (unsafely).

    插入时将会保证时间顺序，但不保证时间不相交。
    Inserting will ensure the time order, but it will not ensure the time does not intersect.

    返回值为实际插入的索引。
    Return the inserted index.

    ```rust
    # use rusty_studio::timeline::{Item,TimeRange,Track};
    # use rusty_studio::core::TimeRangeSupport;
    let mut track = Track::default();
    let item = Item::from_timerange(TimeRange::from_millisecond(35,30));
    track.force_push_item(Box::new(Item::from_timerange(TimeRange::from_millisecond(5,5))));
    track.force_push_item(Box::new(Item::from_timerange(TimeRange::from_millisecond(30,5))));
    track.force_push_item(Box::new(Item::from_timerange(TimeRange::from_millisecond(100,5))));
    let inserted_index = track.force_add_item(Box::new(item));
    assert_eq!(inserted_index,2);
    ```
    */
    pub fn force_add_item(&mut self, item: Box<Item>) -> usize {
        let insert_point = self.find_insert_point(item.as_ref());
        self.items.insert(insert_point, item);
        insert_point
    }

    /**
    尝试向轨道中安全地添加一个片段。
    Try to add an item to the track safely.

    插入时将会保证时间顺序和时间不相交。
    Inserting will ensure the time order and time does not intersect.

    无论成功与否，返回值为可用的插入索引。
    Return the inserted index no matter success or not.
    ```rust
    # use rusty_studio::timeline::{Item,TimeRange,Track};
    # use rusty_studio::core::TimeRangeSupport;
    let mut track = Track::default();
    let item1 = Item::from_timerange(TimeRange::from_millisecond(40,30)); //safe
    let item2 = Item::from_timerange(TimeRange::from_millisecond(30,10)); //unsafe

    track.force_push_item(Box::new(Item::from_timerange(TimeRange::from_millisecond(5,5))));
    track.force_push_item(Box::new(Item::from_timerange(TimeRange::from_millisecond(30,5))));
    track.force_push_item(Box::new(Item::from_timerange(TimeRange::from_millisecond(100,5))));

    let try1 = track.try_add_item(&Box::new(item1));
    assert_eq!(try1,Ok(2));
    let try2 = track.try_add_item(&Box::new(item2));
    assert_eq!(try2,Err(1));
    ```
    */
    pub fn try_add_item(&mut self, item: &Box<Item>) -> Result<usize, usize> {
        let insert_point = self.find_insert_point(item.as_ref());
        let safe = self.check_insert_point(insert_point, item.as_ref());
        if safe {
            self.items.insert(insert_point, item.to_owned());
            self.update_end_cache(item.as_ref());
            Ok(insert_point)
        } else {
            Err(insert_point)
        }
    }

    pub fn take_at(&mut self, index: usize) -> Box<Item> {
        if index >= self.items.len() {
            self.end_cache.replace(None);
        }
        self.items.remove(index)
    }

    pub fn get(&self, index: usize) -> Option<&Box<Item>> {
        self.items.get(index)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl TimeRangeSupport for Track {
    fn start(&self) -> Time {
        Time::default()
    }

    fn duration(&self) -> Time {
        if self.end_cache.borrow().is_none() {
            let mut new_end = Time::default();
            for item in &self.items {
                if item.end() > new_end {
                    new_end = item.end();
                }
            }
            self.end_cache.replace(Some(new_end));
        }
        self.end_cache.borrow().unwrap()
    }
}

impl MetadataSupport for Track {
    fn get_metadata<T: Any + Send + Sync + Clone>(&self, key: &str) -> Option<T> {
        self.metadata.get(key)
    }

    fn set_metadata<T: Any + Send + Sync + Clone>(&mut self, key: &str, value: T) {
        self.metadata.set(key, value);
    }

    fn erase_metadata(&mut self, key: &String) {
        self.metadata.erase(key);
    }

    fn clear_metadata(&mut self) {
        self.metadata.clear();
    }
}
