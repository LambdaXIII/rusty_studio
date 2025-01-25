#![allow(dead_code)]

use std::any::Any;
use std::borrow::Cow;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use std::sync::Arc;


/**
提供基本的元数据存取功能。
Provide basic storage for simple information.

DataBox 本质上是一个 HashMap，但是它可以存取任意类型的信息。
使用字符串键检索或插入数据，数据将会保存。

它只能用于简单地保存一些数据，它并不是一个严谨的数据结构。

DataBox 内部使用 Cow 存储一个 HashMap，实现了内部隐式共享数据，
所以拷贝性能是很高的。

Examples:
```rust
# use rusty_studio::core::DataBox;
let mut data_box = DataBox::default();
data_box.set("key",123);
let value = data_box.get::<i32>("key");
assert_eq!(value,Some(123));

data_box.set("key2",String::from("super!"));
let value = data_box.get::<String>("key2");
assert_eq!(value,Some(String::from("super!")));

let got  = data_box.get::<i32>("key3");
assert_eq!(got,None);

data_box.erase("key");
let got  = data_box.get::<i32>("key");
assert_eq!(got,None);

data_box.clear();
let got  = data_box.get::<i32>("key2");
assert_eq!(got,None);
```
*/
#[derive(Debug,Clone)]
pub struct DataBox {
    data: Cow<'static,HashMap<String, Rc<dyn Any + Send + Sync>>>,
}

impl Default for DataBox {
    fn default() -> Self {
        Self {
            data: Cow::Owned(HashMap::new()),
        }
    }
}

impl DataBox {
    fn new() -> Self {
        Self::default()
    }

    ///根据键获取数据 | Get data by key.
    pub fn get<T>(&self, key: &str) -> Option<T>
    where
        T: Any + Sync + Send + Clone,
    {
        self.data
            .get(key)
            .and_then(|any| any.downcast_ref::<T>().cloned())
    }

    ///保存数据。 | Save data.
    pub fn set<T>(&mut self, key: &str, value: T)
    where
        T: Any + Sync + Send + Clone,
    {
        self.data.to_mut().insert(String::from(key), Rc::new(value));
    }

    pub fn erase(&mut self, key: &str) {
        self.data.to_mut().remove(key);
    }

    pub fn clear(&mut self) {
        self.data.to_mut().clear();
    }
}
