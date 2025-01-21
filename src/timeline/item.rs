#![allow(dead_code)]

use crate::core::{DataBox, MetadataSupport, Time};
use crate::timeline::{ContentSupport, TimeRangeTrait, TimeRangeEditableTrait};
use std::any::Any;
use std::cell::{RefCell, RefMut};
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

/**
Represents a segment on the timeline.

It can be a multimedia segment, a subtitle, or a timeline marker.
The type of the Content is dynamic, so please track it yourself.

Item 表示一个存在于时间线上的片段。

它可以是一个多媒体片段，也可以是一段字幕，或是一个时间线标记。
其Content的类型是动态的，所以在使用时请自行追踪它使用的类型。

Item implemented such traits:
 - `ContentSupport`: support for content.
 - `TimeRange`: provide time range information.
 - `TimeRangeEditable`: support for time range editing.
 - `MetadataSupport`: support for metadata storage.

There is also a `metadata()` function which provided a RefMut way to edit the DataBox inside.
*/
pub struct Item {
    start: Time,
    duration: Time,
    metadata: RefCell<DataBox>,
    content: Option<Rc<dyn Any + Send + Sync>>,
}

impl Item {
    pub fn new() -> Self {
        Self::default()
    }

    ///Construct an Item from a TimeRange.
    pub fn from_timerange<T: TimeRangeTrait>(range: T) -> Self {
        Self {
            start: range.start(),
            duration: range.duration(),
            ..Default::default()
        }
    }

    pub fn metadata(&self) -> RefMut<DataBox> {
        self.metadata.borrow_mut()
    }
}

impl Default for Item {
    fn default() -> Self {
        Self {
            start: Time::new(0),
            duration: Time::new(0),
            metadata: RefCell::new(DataBox::default()),
            content: None,
        }
    }
}

impl Clone for Item {
    fn clone(&self) -> Self {
        Self {
            start: self.start,
            duration: self.duration,
            metadata: RefCell::new(self.metadata.borrow().clone()),
            content: self.content.clone(),
        }
    }
}

impl ContentSupport for Item {
    fn get_content<T>(&self) -> Option<T>
    where
        T: Any + Sync + Send + Clone,
    {
        self.content
            .clone()
            .and_then(|c| c.downcast_ref().and_then(Clone::clone))
    }

    fn set_content<T>(&mut self, content: T)
    where
        T: Any + Sync + Send + Clone,
    {
        self.content = Some(Rc::new(content))
    }

    fn clear_content(&mut self) {
        self.content = None
    }
}

impl TimeRangeTrait for Item {
    fn start(&self) -> Time {
        self.start
    }

    fn duration(&self) -> Time {
        self.duration
    }
}

impl TimeRangeEditableTrait for Item {
    fn set_start(&mut self, start: Time) {
        self.start = start;
    }

    fn set_duration(&mut self, duration: Time) {
        self.duration = duration;
    }
}

impl MetadataSupport for Item {
    fn get_metadata<T: Any + Send + Sync + Clone>(&self, key: &String) -> Option<T> {
        self.metadata.borrow().get(key)
    }

    fn set_metadata<T: Any + Send + Sync + Clone>(&mut self, key: &String, value: T) {
        self.metadata.borrow_mut().set(key, value);
    }

    fn erase_metadata(&mut self, key: &String) {
        self.metadata.borrow_mut().erase(key);
    }

    fn clear_metadata(&mut self) {
        self.metadata.borrow_mut().clear();
    }
}

impl Debug for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Item")
            .field("start", &self.start)
            .field("end", &self.end())
            .field("duration", &self.duration)
            .field(
                "content",
                match &self.content {
                    None => &"None",
                    Some(_) => &"Yes",
                },
            )
            .finish()
    }
}
