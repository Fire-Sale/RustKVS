#![deny(missing_docs)]
//! A key-value store.
use std::collections::HashMap;

#[derive(Default)]
/// KvStore stores <key, value> pairs in a HashMap.
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    /// Creat a new kv store
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    /// Set given <k, v> pair.
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    /// Get value of given key.
    pub fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).cloned()
    }

    /// Remove given key.
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
