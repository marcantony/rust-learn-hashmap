use super::{HashMap, Entry};

pub struct IterMut<'a, 'b, K, V> {
    iterator: Box<dyn Iterator<Item = Entry<&'a K, &'a mut V>> + 'b>
}

impl<K, V> HashMap<K, V> {
    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        IterMut {
            iterator: Box::new(self.items.iter_mut().flatten()
                .map(|entry| Entry { key: &entry.key, value: &mut entry.value }))
        }
    }
}

impl<'a, 'b, K, V> Iterator for IterMut<'a, 'b, K, V> {
    type Item = Entry<&'a K, &'a mut V>;

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
        let mut map_items: Vec<(&i32, &mut i32)> = map.iter_mut()
            .map(|entry| (entry.key, entry.value))
            .collect();
        let mut entries = make_entries();
        let mut processed_entries: Vec<(&i32, &mut i32)> = entries.iter_mut()
            .map(|entry| (&entry.0, &mut entry.1)).collect();

        // Map is unordered, so make sure these are in the same order
        map_items.sort_by_key(|entry| entry.0);
        processed_entries.sort_by_key(|entry| entry.0);

        assert_eq!(map_items, processed_entries);
    }

    #[test]
    fn test_iter_mutation() {
        let mut map = HashMap::new();

        map.put(1, 1);

        {
            let mut iter = map.iter_mut();
            let some_entry = iter.next().unwrap();
            *some_entry.value = 2;
        }

        assert_eq!(map.get(&1), Some(&2));
    }

    #[test]
    fn cannot_mutate_key() {
        let mut map = HashMap::new();

        map.put(1, 1);

        {
            let mut iter = map.iter_mut();
            let mut some_entry = iter.next().unwrap();
            some_entry.key = &6;
        }

        assert_eq!(map.get(&1), Some(&1));
    }
}
