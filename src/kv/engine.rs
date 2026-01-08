use std::collections::HashMap;
use crate::kv::command::Command; 

// could be a field of the actual server object
pub trait Store {
    fn apply(&mut self, cmd: Command) -> Option<String>; // TODO!: change to Vec<u8> later
}

pub struct InMemoryStore {
    store: HashMap<String, String>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),

        }
    }
}

impl Store for InMemoryStore {
    fn apply(&mut self, cmd: Command) -> Option<String> {
        if let Command::Put(key, value) = cmd {
            self.store.insert(key,value); // if key already exists, this is an update
        } else if let Command::Get(key) = cmd {
            if self.store.contains_key(&key) {
                return self.store.get(&key).cloned();
            } else {
                return None
            }
        } else if let Command::Delete(key) = cmd {
            let removed_value = self.store.remove(&key);
            return removed_value;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_in_memory_get() {
        let mut store = HashMap::new();
        store.insert("key".to_string(), "value".to_string());
        let mut store = InMemoryStore {
            store,
        };
        assert_eq!(store.apply(Command::Get("key".to_string())), Some("value".to_string()));
        assert_eq!(store.apply(Command::Get("key".to_string())), Some("value".to_string()));
    }

    #[test]
    fn test_in_memory_put() {
        let mut store = InMemoryStore {
            store: HashMap::new(),
        };
        assert_eq!(None, store.apply(Command::Put("key".to_string(), "value".to_string())));
        assert_eq!(Some("value".to_string()),store.apply(Command::Get("key".to_string())));
        assert_eq!(Some("value".to_string()),store.apply(Command::Get("key".to_string())));
    }

    #[test]
    fn tests_in_memory_delete() {
        let mut store = InMemoryStore {
            store: HashMap::new(),
        };
        assert_eq!(None, store.apply(Command::Put("key1".to_string(), "value1".to_string())));
        assert_eq!(None, store.apply(Command::Put("key2".to_string(), "value2".to_string())));
        assert_eq!(None, store.apply(Command::Put("key3".to_string(), "value3".to_string())));

        assert_eq!(Some("value2".to_string()), store.apply(Command::Delete("key2".to_string())));
        assert_eq!(Some("value3".to_string()), store.apply(Command::Delete("key3".to_string())));

        assert_eq!(Some("value1".to_string()),store.apply(Command::Get("key1".to_string())));
        assert_eq!(Some("value1".to_string()),store.apply(Command::Get("key1".to_string())));
    }

    #[test]
    fn test_in_memory_empty_delete() {
        let mut store = InMemoryStore {
            store: HashMap::new(),
        };
        assert_eq!(None, store.apply(Command::Delete("key".to_string())));
        assert_eq!(None, store.apply(Command::Delete("key".to_string())));

        assert_eq!(None,store.apply(Command::Get("key".to_string())));
        assert_eq!(None,store.apply(Command::Get("key".to_string())));
    }
}
