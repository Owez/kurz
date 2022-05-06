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
        let max = max.into();
        if max.is_some() {
            // Fix capacity for initial perf if max is defined
            Self {
                inner: HashMap::with_capacity(prune_len(max).unwrap()),
                old: Vec::with_capacity(prune_over(max).unwrap()),
                max,
            }
        } else {
            // No max is defined, don't add initial capacity
            Self {
                inner: HashMap::new(),
                old: vec![],
                max,
            }
        }
    }

    /// Inserts new item into cache
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        // Check if it should be added to `old`
        match prune_over(self.max) {
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
        match prune_len(self.max) {
            Some(len) if self.inner.len() > len => {
                // TODO: capacity isnt reducing
                trace!("Pruning cache because it's 20% over");
                for k in &self.old {
                    self.inner.remove(k);
                }
                self.old.clear();
            }
            _ => (),
        }
    }
}

/// Value (if any) defined which causes the cache to be pruned
fn prune_len(max: Option<usize>) -> Option<usize> {
    max.map(|val| val + prune_over(max).unwrap())
}

/// Amount maximum is allowed to be over, used for [Self::prune_len] calculations
fn prune_over(max: Option<usize>) -> Option<usize> {
    max.map(|val| (val as f64 * 0.2) as usize)
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
        assert_eq!(cache.inner.capacity(), 1200)
    }
}
