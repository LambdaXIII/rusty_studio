#![allow(dead_code)]

use super::TimeRange;
use crate::core::Time;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrackNoSafeInsertionError;

impl std::fmt::Display for TrackNoSafeInsertionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "No safe insertion point found.")
    }
}
impl std::error::Error for TrackNoSafeInsertionError {}

/**
Track 结构体用于维护一条时间线轨道，其中可以按顺序保存多个 TimeRange 对象。

虽然此模块中也提供了 TimelineItem 结构体，但是 Track 并不要求片段用它实现，
Track 可以保存任何 TimeRange 对象。

Track 对象内部使用 Vec<Box<dyn TimeRange>> 保存对象，
并作品提供了一系列方法用于检索、插入、删除对象，并根据时间点确保它们按顺序保存，
避免了随时可能需要排序的问题。
*/
pub struct Track {
    pub name: String,
    pub description: String,
    items: Vec<Box<dyn TimeRange>>,
}

impl Track {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            items: Vec::new(),
        }
    }
    
    pub fn from_items(items:Vec<Box::<dyn TimeRange>>) -> Self{
        Self{
            name:String::new(),
            description:String::new(),
            items
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /**
    为对象查找合适的插入点。
    如果按照返回的索引值插入对象，则一定能保证片段的开始时间在正确的顺序中。
但是并不能保证插入的片段和前后片段交叉。

    所以正式使用之后应当使用其它方法继续检查此插入点是否安全。
    **/
    pub fn find_insert_point(&self, item: &Box<dyn TimeRange>) -> usize {
        self.items.partition_point(|i| i.start() < item.start())
    }

    /**
    用于检查插入点是否可以安全插入片段。

    此方法仅检测插入点前后的片段是否与待插入的片段交叉，
    并不检查片段的顺序，所以应当使用 find_insert_point 返回的插入索引。
    */
    pub fn check_insert_point(&self, index: usize, item: &Box<dyn TimeRange>) -> bool {
        let mut items: Vec<&Box<dyn TimeRange>> = Vec::new();
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

    /**
    按照索引强制插入对象，
    如果使用经过前两个方法检查过的索引值，则插入是安全的。
        */
    pub fn insert_item(&mut self, index: usize, item: Box<dyn TimeRange>) {
        self.items.insert(index, item);
    }

    pub fn force_add_item(&mut self, item: Box<dyn TimeRange>) {
        let index = self.find_insert_point(&item);
        self.insert_item(index, item);
    }

    /**
    使用前面的几个方法尝试安全插入对象。
    如果无法安全插入，则会返回错误结果。
    如果成功插入，则返回索引值。
    */
    pub fn try_add_item(
        &mut self,
        item: Box<dyn TimeRange>,
    ) -> Result<usize, TrackNoSafeInsertionError> {
        let index = self.find_insert_point(&item);
        if self.check_insert_point(index, &item) {
            self.insert_item(index, item);
            return Ok(index);
        }
        Err(TrackNoSafeInsertionError)
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn sort_items(&mut self) {
        self.items.sort_by(|a, b| a.start().cmp(&b.start()));
    }

    pub fn items(&self) -> &Vec<Box<dyn TimeRange>> {
        &self.items
    }
}

impl TimeRange for Track {
    fn start(&self) -> Time {
        Time::new(0)
    }

    fn duration(&self) -> Time {
        if self.items.is_empty() {
            return Time::new(0);
        }
        self.items.last().unwrap().end()
    }
}

impl Default for Track{
    fn default() -> Self {
        Self{
            name:String::new(),
            description:String::new(),
            items:Vec::new(),
        }
    }
}