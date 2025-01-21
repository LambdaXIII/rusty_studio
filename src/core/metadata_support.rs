use std::any::Any;

/**
Provide metadata storage functions.

Using `DataBox` to implement this trait is recommended.
*/
pub trait MetadataSupport {
    fn get_metadata<T: Any + Send + Sync + Clone>(&self, key: &String) -> Option<T>;
    fn set_metadata<T: Any + Send + Sync + Clone>(&mut self, key: &String, value: T);
    fn erase_metadata(&mut self, key: &String);
    fn clear_metadata(&mut self);
}
