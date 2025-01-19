#![allow(dead_code)]

use std::collections::HashMap;

pub trait MetadataSupport {
    fn set_metadata(&mut self, key: String, value: String);
    fn get_metadata(&self, key: &String) -> Option<&String>;
    fn get_all_metadata(&self) -> Box<HashMap<String, String>>;
}
