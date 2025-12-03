use std::collections::HashMap;
use std::hash::Hash;

/// Simple cache implementation
pub struct Cache<K, V> {
    data: HashMap<K, V>,
    max_size: usize,
}

impl<K, V> Cache<K, V>
where
    K: Hash + Eq,
{
    pub fn new(max_size: usize) -> Self {
        Self {
            data: HashMap::new(),
            max_size,
        }
    }
    
    pub fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }
    
    pub fn insert(&mut self, key: K, value: V) {
        if self.data.len() >= self.max_size {
            // Simple eviction: clear cache if full
            // Note: In production, use a better eviction strategy (LRU, etc.)
            if self.data.len() >= self.max_size {
                self.data.clear();
            }
        }
        self.data.insert(key, value);
    }
    
    pub fn clear(&mut self) {
        self.data.clear();
    }
    
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

