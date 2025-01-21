use super::{Item, TimeRangeTrait};
use crate::core::{DataBox,Time};
use std::cell::RefCell;

pub struct Track {
    items: Vec<Box<Item>>,
    metadata: RefCell<DataBox>,
    end_cache: Option<Time>,
}

impl Default for Track {
    fn default() -> Self {
        Self {
            items: vec![],
            metadata: RefCell::new(DataBox::default()),
            end_cache: None,
        }
    }
}

impl Clone for Track {
    fn clone(&self) -> Self {
        Self {
            items: self.items.clone(),
            metadata: RefCell::new(self.metadata.borrow().clone()),
            end_cache: None,
        }
    }
}

impl Track {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn force_push_item(&mut self, item: Box<Item>) {
        self.items.push(item);
        self.end_cache = None;
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
        self.items.insert(insert_point, item);
        self.end_cache = None;
        if insert_point == index {
            Ok(insert_point)
        } else {
            Err(insert_point)
        }
    }

    pub fn force_sort_items(&mut self) {
        self.items.sort_by(|a, b| a.start().cmp(&b.start()));
        self.end_cache = self.items.last().and_then(|x| Some(x.end()));
    }

    /**
    根据时间段信息查找预备插入位置。
    Search for insert point for a time range.

    ```rust
    # use rusty_studio::timeline::{Item,TimeRange,TimeRangeTrait,Track};
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
    pub fn find_insert_point(&self, item: &dyn TimeRangeTrait) -> usize {
        if self.items.is_empty() {
            return 0;
        }
        if self.end_cache.is_some() && self.end_cache.unwrap() < item.start() {
            return self.items.len();
        }

        let search = self
            .items
            .binary_search_by(|x| x.start().cmp(&item.start()));
        search.unwrap_or_else(|index| index)
    }
}
