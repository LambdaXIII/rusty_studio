#![allow(dead_code)]

use std::any::Any;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;


/**
Provide basic storage for simple information.

DataBox 本质上是一个 HashMap，但是它可以存取任意类型的信息。
使用字符串键检索或插入数据，数据将会保存。

它只能用于简单地保存一些数据，它并不是一个严谨的数据结构。

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
    data_ref: Box<HashMap<String, Arc<dyn Any + Send + Sync>>>,
}

impl Default for DataBox {
    fn default() -> Self {
        Self {
            data_ref: Box::new(HashMap::new()),
        }
    }
}

impl DataBox {
    fn new() -> Self {
        Self::default()
    }

    ///根据键获取数据。
    pub fn get<T>(&self, key: &str) -> Option<T>
    where
        T: Any + Sync + Send + Clone,
    {
        self.data_ref
            .get(key)
            .and_then(|any| any.downcast_ref::<T>().cloned())
    }

    ///保存数据。
    pub fn set<T>(&mut self, key: &str, value: T)
    where
        T: Any + Sync + Send + Clone,
    {
        self.data_ref.insert(String::from(key), Arc::new(value));
    }

    pub fn erase(&mut self, key: &str) {
        self.data_ref.remove(key);
    }

    pub fn clear(&mut self) {
        self.data_ref.clear();
    }
}

impl<T> From<HashMap<String, Arc<T>>> for DataBox
where
    T: Any + Sync + Send + Clone,
{
    fn from(data: HashMap<String, Arc<T>>) -> Self {
        let mut result = Self::default();
        for (k, v) in data.into_iter() {
            result.data_ref.insert(k, v.clone() as Arc<dyn Any + Send + Sync>);
        }
        result
    }
}
