use std::any::Any;

pub trait ContentSupport {
    fn get_content<T>(&self) -> Option<T>
    where
        T: Any + Sync + Send + Clone;

    fn set_content<T>(&mut self, content: T)
    where
        T: Any + Sync + Send + Clone;

    fn clear_content(&mut self);
}

