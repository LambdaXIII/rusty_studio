#![allow(dead_code)]

use crate::core::{DataBox, MetadataSupport, Time};
use crate::timeline::{ContentSupport, TimeRangeEditableTrait, TimeRangeTrait};
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

/**
提供以*任意*类型保存片段内容的支持。
Provide support for store a content in Any type.

Exp1 基本操作:
```rust
# use rusty_studio::timeline::{Item,ContentSupport};
let mut item = Item::new();
item.set_content::<i32>(123);
assert_eq!(item.get_content::<i32>(), Some(123));
item.set_content(String::from("Hello World!"));
assert_eq!(item.get_content::<String>(), Some(String::from("Hello World!")));
item.clear_content();
assert_eq!(item.get_content::<String>(), None);
```

Exp2 使用自定义类型:
```rust
# use rusty_studio::timeline::{Item,ContentSupport};
#[derive(Debug, Clone, Eq, PartialEq)]
struct RGBColor {
    r:u8,g:u8,b:u8
}

let color = RGBColor { r: 255, g: 128, b: 32 };
let mut item = Item::default();
item.set_content(color.clone());
assert_eq!(item.get_content::<RGBColor>(), Some(color));
```
*/
impl ContentSupport for Item {
    fn get_content<T>(&self) -> Option<T>
    where
        T: Any + Sync + Send + Clone,
    {
        self.content.clone()
            .and_then(|rc| rc.downcast_ref::<T>().cloned())
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

/**
Access metadata storage using these functions.

Example:
```rust
# use rusty_studio::timeline::Item;
# use rusty_studio::core::MetadataSupport;
let mut item = Item::new();
item.set_metadata("number1", 123);
item.set_metadata("number2",456.78);
item.set_metadata("note", String::from("This is a note"));
assert_eq!(item.get_metadata::<i32>("number1"), Some(123));
assert_eq!(item.get_metadata::<f64>("number2"), Some(456.78));
assert_eq!(item.get_metadata::<String>("note"), Some(String::from("This is a note")));
assert_eq!(item.get_metadata::<i32>("unknown metadata"), None);
```
*/
impl MetadataSupport for Item {
    fn get_metadata<T: Any + Send + Sync + Clone>(&self, key: &str) -> Option<T> {
        self.metadata.borrow().get(key)
    }

    fn set_metadata<T: Any + Send + Sync + Clone>(&mut self, key: &str, value: T) {
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
