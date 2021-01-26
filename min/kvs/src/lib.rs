//! A simple key/value store
//!

#![deny(missing_docs)]

use std::{collections::HashMap, option::Option};
/// A key/value store
pub struct KvStore {
    inner: HashMap<String, String>,
}

impl KvStore {
    /// Create a new key/value store
    /// ```rust
    /// use kvs::KvStore;
    /// fn main() {
    ///     let store = KvStore::new();
    /// #   assert_eq!("test", "test")
    /// }
    /// ```
    pub fn new() -> KvStore {
        KvStore {
            inner: HashMap::new(),
        }
    }

    /// Set the key/value pair into the store
    pub fn set(&mut self, key: String, value: String) {
        self.inner.insert(key, value);
    }

    /// Get the value of the specified key
    pub fn get(&self, key: String) -> Option<String> {
        if let Some(value) = self.inner.get(&key) {
            return Some(String::from(value));
        }
        None
    }

    /// Remove the key/value pair from the store
    pub fn remove(&mut self, key: String) {
        self.inner.remove(&key);
    }
}
