use std::option::Option;
pub struct KvStore {}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {}
    }

    pub fn set(&mut self, key: String, value: String) {
        panic!("")
    }

    pub fn get(&self, key: String) -> Option<String> {
        panic!("")
    }

    pub fn remove(&mut self, key: String) {
        panic!("")
    }
}
