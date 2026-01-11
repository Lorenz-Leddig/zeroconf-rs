//! Windows implementation for TXT records.

use crate::Result;
use crate::txt_record::TTxtRecord;

#[cfg(all(target_os = "windows", feature = "windows-native"))]
pub struct WindowsTxtRecord;

#[cfg(all(target_os = "windows", feature = "windows-native"))]
impl TTxtRecord for WindowsTxtRecord {
    fn new() -> Self {
        todo!()
    }

    fn insert(&mut self, _key: &str, _value: &str) -> Result<()> {
        todo!()
    }

    fn get(&self, _key: &str) -> Option<String> {
        todo!()
    }

    fn remove(&mut self, _key: &str) -> Option<String> {
        todo!()
    }

    fn contains_key(&self, _key: &str) -> bool {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (String, String)> + 'a> {
        todo!()
    }

    fn keys<'a>(&'a self) -> Box<dyn Iterator<Item = String> + 'a> {
        todo!()
    }

    fn values<'a>(&'a self) -> Box<dyn Iterator<Item = String> + 'a> {
        todo!()
    }
}

#[cfg(all(target_os = "windows", feature = "windows-native"))]
impl Clone for WindowsTxtRecord {
    fn clone(&self) -> Self {
        todo!()
    }
}

#[cfg(all(target_os = "windows", feature = "windows-native"))]
impl PartialEq for WindowsTxtRecord {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}
