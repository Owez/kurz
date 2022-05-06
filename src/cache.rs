//! Contains [Cache] and implementations

use log::{info, trace};
use std::{collections::HashMap, hash::Hash};

/// Locally cached key-value store
pub struct Cache<K: Eq + Hash + Clone, V> {
    /// Items within this cache
    pub inner: HashMap<K, V>,
    /// The oldest 20% of items in cache
    pub old: Vec<K>,
    /// Optional indented length; actual maximum length may be up to 20% higher
    pub max: Option<usize>,
}

impl<K: Eq + Hash + Clone, V> Cache<K, V> {
    /// Initializes new cache with optional `max` length
    pub fn new(max: impl Into<Option<usize>>) -> Self {
        info!("Initializing cache");

        // Create cache
        let max = max.into();
        let mut cache = Self {
            inner: HashMap::new(),
            old: vec![],
            max,
        };

        // Add capacity to old
        if let Some(old_cap) = cache.prune_over() {
            cache.old = Vec::with_capacity(old_cap);
        }
        cache
    }

    /// Inserts new item into cache
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        // Check if it should be added to `old`
        match self.prune_over() {
            Some(over) if self.old.len() < over => self.old.push(k.clone()),
            _ => (),
        }

        // Prune and insert
        self.prune();
        self.inner.insert(k, v)
    }

    /// Prunes cache to [Self::max] if it's 20% over
    fn prune(&mut self) {
        trace!("Checking if cache needs pruning");
        match self.prune_len() {
            Some(len) if self.inner.len() > len => {
                trace!("Pruning cache because it's 20% over");
                for k in &self.old {
                    self.inner.remove(k);
                }
                self.old.clear();
            }
            _ => (),
        }
    }

    /// Value (if any) defined which causes the cache to be pruned
    fn prune_len(&self) -> Option<usize> {
        self.max.map(|max| max + self.prune_over().unwrap())
    }

    /// Amount maximum is allowed to be over, used for [Self::prune_len] calculations
    fn prune_over(&self) -> Option<usize> {
        self.max.map(|max| (max as f64 * 0.2) as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks we can insert a ton of items and then prune it automatically
    #[test]
    fn inserts_prune() {
        let mut cache = Cache::new(1000);
        for i in 0..500000 {
            cache.insert(i, i * 2);
        }
    }
}
