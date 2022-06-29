use super::{HashMap, Entry};

pub struct Iter<'a, 'b, K: 'a, V: 'a> {
    iterator: Box<dyn Iterator<Item = &'a Entry<K, V>> + 'b>
}

impl<'a, K, V> HashMap<K, V> {
    pub fn iter(&'a self) -> Iter<K, V> {
        Iter {
            iterator: Box::new(self.items.iter().flatten())
        }
    }
}

impl<'a, 'b, K: 'a, V: 'a> Iterator for Iter<'a, 'b, K, V> {
    type Item = &'a Entry<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entries() -> Vec<(i32, i32)> {
        let limit = 100;
        (1..limit).zip(1..limit).collect()
    }

    #[test]
    fn test_iterator() {
        let mut map = HashMap::new();

        // Make sure multiple buckets in map are filled
        for entry in make_entries() {
            map.put(entry.0, entry.1);
        }

        // Use iterator to pull out all items
        let mut map_items: Vec<(i32, i32)> = map.iter()
            .map(|entry| (entry.key, entry.value))
            .collect();
        let mut generated_entries = make_entries();

        // Map is unordered, so make sure these are in the same order
        map_items.sort_by_key(|entry| entry.0);
        generated_entries.sort_by_key(|entry| entry.0);

        assert_eq!(map_items, generated_entries);
    }
}
