use std::any::Any;

/**
提供基础的元数据存取功能。
Provide metadata storage functions.

建议使用 `DataBox` 实现此功能。
Using `DataBox` to implement this trait is recommended.
*/
pub trait MetadataSupport {
    fn get_metadata<T: Any + Send + Sync + Clone>(&self, key: &str) -> Option<T>;
    fn set_metadata<T: Any + Send + Sync + Clone>(&mut self, key: &str, value: T);
    fn erase_metadata(&mut self, key: &String);
    fn clear_metadata(&mut self);
}
