use std::any::Any;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

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

    pub fn get<T>(&self, key: &str) -> Option<T>
    where
        T: Any + Sync + Send + Clone,
    {
        self.data_ref
            .get(key)
            .and_then(|any| any.downcast_ref::<T>().cloned())
    }

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

impl Clone for DataBox {
    fn clone(&self) -> Self {
        Self {
            data_ref: self.data_ref.to_owned(),
        }
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
