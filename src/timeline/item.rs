#![allow(dead_code)]

use crate::core::{DataBox, MetadataSupport, Time};
use crate::core::{TimeRangeEditingSupport, TimeRangeSupport};
use std::any::Any;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

/**
表示一个存在于时间线上的片段。
Represents a segment on the timeline.

它可以是一个多媒体片段，也可以是一段字幕，或是一个时间线标记。
其Content的类型是动态的，所以在使用时请自行追踪它使用的类型。

It can be a multimedia segment, a subtitle, or a timeline marker.
The type of the Content is dynamic, so please track it yourself.

它实现了这些Trait：
 - `ContentSupport`: 支持存取多种类型的`内容`
 - `TimeRange`: 提供时间范围信息。
 - `TimeRangeEditable`: 支持时间范围的编辑。
 - `MetadataSupport`: 支持存取元数据。

Item implemented such traits:
 - `ContentSupport`: support for content.
 - `TimeRange`: provide time range information.
 - `TimeRangeEditable`: support for time range editing.
 - `MetadataSupport`: support for metadata storage.
*/
pub struct Item {
    start: Time,
    duration: Time,
    metadata: DataBox,
    content: Option<Rc<dyn Any + Send + Sync>>,
}

impl Item {
    pub fn new<T>(start: i128, duration: i128, content: T) -> Self
    where
        T: Any + Send + Sync + Clone,
    {
        Self {
            start: Time::new(start),
            duration: Time::new(duration),
            metadata: DataBox::default(),
            content: Some(Rc::new(content)),
        }
    }

    ///从另一个 TimeRangeTrait 对象构造一个空的片段 | Construct an Item from a TimeRange.
    pub fn from_timerange<T: TimeRangeSupport>(range: T) -> Self {
        Self {
            start: range.start(),
            duration: range.duration(),
            ..Default::default()
        }
    }

    pub fn get_content<T>(&self) -> Option<T>
    where
        T: Any + Sync + Send + Clone,
    {
        self.content
            .clone()
            .and_then(|rc| rc.downcast_ref::<T>().cloned())
    }

    pub fn set_content<T>(&mut self, content: T)
    where
        T: Any + Sync + Send + Clone,
    {
        self.content = Some(Rc::new(content))
    }

    pub fn clear_content(&mut self) {
        self.content = None
    }
}

impl Default for Item {
    fn default() -> Self {
        Self {
            start: Time::new(0),
            duration: Time::new(0),
            metadata: DataBox::default(),
            content: None,
        }
    }
}

impl Clone for Item {
    fn clone(&self) -> Self {
        Self {
            start: self.start,
            duration: self.duration,
            metadata: self.metadata.clone(),
            content: self.content.clone(),
        }
    }
}

/**
提供以*任意*类型保存片段内容的支持。
Provide support for store a content in Any type.

Exp1 基本操作:
```rust
# use rusty_studio::timeline::Item;
let mut item = Item::default();
item.set_content::<i32>(123);
assert_eq!(item.get_content::<i32>(), Some(123));
item.set_content(String::from("Hello World!"));
assert_eq!(item.get_content::<String>(), Some(String::from("Hello World!")));
item.clear_content();
assert_eq!(item.get_content::<String>(), None);
```

Exp2 使用自定义类型:
```rust
# use rusty_studio::timeline::Item;
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


impl TimeRangeSupport for Item {
    fn start(&self) -> Time {
        self.start
    }

    fn duration(&self) -> Time {
        self.duration
    }
}

impl TimeRangeEditingSupport for Item {
    fn set_start(&mut self, start: Time) {
        self.start = start;
    }

    fn set_duration(&mut self, duration: Time) {
        self.duration = duration;
    }
}

/**
提供存取元数据的支持。
Access metadata storage using these functions.

Example:
```rust
# use rusty_studio::timeline::Item;
# use rusty_studio::core::MetadataSupport;
let mut item = Item::default();
item.set_metadata("number1", 123);
item.set_metadata("number2",456.78);
item.set_metadata("note", String::from("This is a note"));
assert_eq!(item.get_metadata::<i32>("number1"), Some(123));
assert_eq!(item.get_metadata::<f64>("number2"), Some(456.78));
assert_eq!(item.get_metadata::<String>("note"), Some(String::from("This is a note")));
assert_eq!(item.get_metadata::<i32>("unknown metadata"), None);
```
*要不是为了单元测试，我才不想写这些示例代码呢*
*/
impl MetadataSupport for Item {
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
