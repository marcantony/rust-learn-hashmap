use std::{hash::Hash, marker::PhantomData};

pub struct HashMap<K, V> {
    key: PhantomData<K>,
    value: PhantomData<V>
}

impl<K: Hash, V> HashMap<K, V> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        todo!()
    }

    pub fn put(&mut self, key: K, value: V) {
        todo!()
    }

    pub fn pop(&mut self, key: &K) -> Option<V> {
        todo!()
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
        map.put("foo", "2");
        assert_eq!(map.get(&"foo"), Some(&"2"));
    }
}
