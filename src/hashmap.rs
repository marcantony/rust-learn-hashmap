use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher, mem};

pub mod iter;
pub mod iter_mut;
pub mod into_iter;

pub struct HashMap<K, V> {
    items: Vec<Vec<Entry<K, V>>>,
    size: usize
}

pub struct Entry<K, V> {
    pub key: K,
    pub value: V
}

const DEFAULT_CAPACITY: usize = 16;
const DEFAULT_LOAD_FACTOR: f64 = 0.75;

fn hash(value: &impl Hash) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

fn find_key_index(key: &impl Hash, capacity: usize) -> usize {
    let h = hash(&key);
    // "as" here is fine since we're truncating the hash with the modulo anyway
    h as usize % capacity
}

impl<K: Hash + Eq, V> HashMap<K, V> {
    fn create_backing_vec(capacity: usize) -> Vec<Vec<Entry<K, V>>> {
        let mut vec = Vec::with_capacity(capacity);
        vec.resize_with(capacity, Vec::new);
        vec
    }

    pub fn new() -> Self {
        HashMap::with_capacity(DEFAULT_CAPACITY)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let vec = HashMap::create_backing_vec(capacity);
        HashMap { items: vec, size: 0 }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let index = find_key_index(&key, self.capacity());
        let containing_list = &self.items[index];

        containing_list.iter()
            .find(|entry| &entry.key == key)
            .map(|entry| &entry.value)
    }

    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        let index = find_key_index(&key, self.capacity());
        let containing_list = &mut self.items[index];

        let existing_entry = containing_list.iter_mut()
            .find(|entry| entry.key == key);

        let existing_value = match existing_entry {
            Some(entry) => Some(mem::replace(&mut entry.value, value)),
            None => {
                let new_entry = Entry { key: key, value: value };
                containing_list.push(new_entry);
                self.size += 1;
                None
            }
        };

        if self.exceeds_threshold() {
            self.resize(self.capacity() * 2);
        }

        existing_value
    }

    pub fn pop(&mut self, key: &K) -> Option<V> {
        let index = find_key_index(&key, self.capacity());
        let containing_list = &mut self.items[index];

        containing_list.iter()
            .position(|entry| &entry.key == key)
            .map(|position| {
                self.size -= 1;
                containing_list.swap_remove(position).value
            })
    }

    pub fn resize(&mut self, capacity: usize) {
        let mut new_vec: Vec<Vec<Entry<K, V>>> = HashMap::create_backing_vec(capacity);
        for entry in mem::take(&mut self.items).into_iter().flatten() {
            let index = find_key_index(&entry.key, new_vec.len());
            new_vec[index].push(entry)
        }
        self.items = new_vec;
    }

    pub fn size(&self) -> usize {
        self.size
    }

    fn capacity(&self) -> usize {
        self.items.len()
    }

    fn exceeds_threshold(&self) -> bool {
        self.size() as f64 >= (self.capacity() as f64) * DEFAULT_LOAD_FACTOR
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

    #[test]
    fn test_resize() {
        let mut map = HashMap::with_capacity(DEFAULT_CAPACITY);

        let entries: Vec<(String, i32)> = (1..100).map(|i| i.to_string()).zip(1..100).collect();

        // Fill map
        for entry in entries.iter() {
            map.put(&entry.0[..], entry.1);
        }

        // Resizing map larger doesn't mess up keys
        map.resize(100);
        for entry in entries.iter() {
            assert_eq!(map.get(&&entry.0[..]), Some(&entry.1))
        }

        // Shrinking map doesn't mess up keys
        map.resize(2);
        for entry in entries.iter() {
            assert_eq!(map.get(&&entry.0[..]), Some(&entry.1))
        }
    }

    #[test]
    fn test_size() {
        let mut map = HashMap::new();

        assert_eq!(map.size(), 0);

        map.put("key", 1);
        assert_eq!(map.size(), 1);

        map.pop(&"key");
        assert_eq!(map.size(), 0);

        map.pop(&"key");
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_dynamic_resizing() {
        let initial_capacity = 16;
        let mut map = HashMap::with_capacity(initial_capacity);

        assert_eq!(map.capacity(), initial_capacity);

        let entries: Vec<(String, i32)> = (1..100).map(|i| i.to_string()).zip(1..100).collect();

        for entry in entries.iter() {
            map.put(&entry.0[..], entry.1);
        }

        assert!(map.capacity() > initial_capacity);

        for entry in entries.iter() {
            assert_eq!(map.get(&&entry.0[..]), Some(&entry.1))
        }
    }
}
