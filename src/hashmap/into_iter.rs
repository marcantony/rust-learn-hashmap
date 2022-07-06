use super::{HashMap, Entry};

/// An [Iterator] for a [HashMap] which returns its entries with ownership.
pub struct IntoIter<'a, K, V> {
    iterator: Box<dyn Iterator<Item = Entry<K, V>> + 'a>
}

impl<'a, K: 'a, V: 'a> HashMap<K, V> {
    /// Consume this [HashMap] to produce an iterator.
    pub fn into_iter(self) -> IntoIter<'a, K, V> {
        IntoIter {
            iterator: Box::new(self.items.into_iter().flatten())
        }
    }
}

impl<'a, K, V> Iterator for IntoIter<'a, K, V> {
    type Item = Entry<K, V>;

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
        let mut map_items: Vec<(i32, i32)> = map.into_iter()
            .map(|entry| (entry.key, entry.value))
            .collect();
        let entries = make_entries();
        let mut processed_entries: Vec<(i32, i32)> = entries.into_iter().collect();

        // Map is unordered, so make sure these are in the same order
        map_items.sort_by_key(|entry| entry.0);
        processed_entries.sort_by_key(|entry| entry.0);

        assert_eq!(map_items, processed_entries);
    }
}
