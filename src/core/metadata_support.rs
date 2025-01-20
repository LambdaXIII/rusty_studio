use std::any::Any;

pub trait MetadataSupport {
    fn get_metadata<T: Any + Send + Sync + Clone>(&self, key: &String) -> Option<T>;
    fn set_metadata<T: Any + Send + Sync + Clone>(&mut self, key: &String, value: T);
    fn erase_metadata(&mut self, key: &String);
    fn clear_metadata(&mut self);
}
