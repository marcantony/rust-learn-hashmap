use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher, mem};

pub mod iter;

pub struct HashMap<K, V> {
    items: Vec<Vec<Entry<K, V>>>
}

pub struct Entry<K, V> {
    pub key: K,
    pub value: V
}

const DEFAULT_SIZE: usize = 16;

fn hash(value: &impl Hash) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

impl<K: Hash + Eq, V> HashMap<K, V> {
    pub fn new() -> Self {
        let mut vec = Vec::with_capacity(DEFAULT_SIZE);
        vec.resize_with(DEFAULT_SIZE, Vec::new);
        HashMap { items: vec }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self.find_key_index(&key);
        let containing_list = &self.items[index];

        containing_list.iter()
            .find(|entry| &entry.key == key)
            .map(|entry| &entry.value)
    }

    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        let index = self.find_key_index(&key);
        let containing_list = &mut self.items[index];

        let existing_entry = containing_list.iter_mut()
            .find(|entry| entry.key == key);

        match existing_entry {
            Some(entry) => Some(mem::replace(&mut entry.value, value)),
            None => {
                let new_entry = Entry { key: key, value: value };
                containing_list.push(new_entry);
                None
            }
        }
    }

    pub fn pop(&mut self, key: &K) -> Option<V> {
        let index = self.find_key_index(&key);
        let containing_list = &mut self.items[index];

        containing_list.iter()
            .position(|entry| &entry.key == key)
            .map(|position| containing_list.swap_remove(position).value)
    }

    fn find_key_index(&self, key: &K) -> usize {
        let h = hash(&key);
        let current_size = self.items.len();
        // "as" here is fine since we're truncating the hash with the modulo anyway
        h as usize % current_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_put() {
        let mut map: HashMap<&str, &str> = HashMap::new();

        // value is None when not present
        assert_eq!(map.get(&"foo"), None);

        // verify put and get
        map.put("foo", "1");
        assert_eq!(map.get(&"foo"), Some(&"1"));

        // verify that another key/value pair works
        map.put("bar", "2");
        assert_eq!(map.get(&"bar"), Some(&"2"));
    }

    #[test]
    fn test_pop() {
        let mut map = HashMap::new();

        map.put("foo", "1");
        assert_eq!(map.pop(&"foo"), Some("1"));
        assert_eq!(map.get(&"foo"), None);
        assert_eq!(map.pop(&"foo"), None);
    }

    #[test]
    fn test_put_overwrite() {
        let mut map = HashMap::new();

        map.put("foo", "1");
        assert_eq!(map.put("foo", "2"), Some("1"));
        assert_eq!(map.get(&"foo"), Some(&"2"));
    }

    #[derive(PartialEq, Eq)]
    struct MyKey {
        foo: i32
    }

    impl MyKey {
        pub fn new(val: i32) -> Self { MyKey { foo: val } }
    }

    impl Hash for MyKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            state.write_i32(1); // Always give same hash
        }
    }

    #[test]
    fn test_keys_colliding_hash () {
        let mut map = HashMap::new();

        // Sanity check that hashes are the same
        assert_eq!(hash(&MyKey::new(1)), hash(&MyKey::new(2)));

        // Insert two different K->V pairs with same hash
        assert_eq!(map.put(MyKey::new(1), "1"), None);
        assert_eq!(map.put(MyKey::new(2), "2"), None);

        assert_eq!(map.get(&MyKey::new(1)), Some(&"1"));
        assert_eq!(map.get(&MyKey::new(2)), Some(&"2"));
    }
}
