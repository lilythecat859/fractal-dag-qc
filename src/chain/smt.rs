use crate::crypto::{Hash, hash};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct SMT {
    pub root: Hash,
    pub store: HashMap<Hash, (Hash, Hash)>,
}

impl Default for SMT {
    fn default() -> Self {
        Self { root: [0; 32], store: HashMap::new() }
    }
}

impl SMT {
    pub fn new() -> Self { Default::default() }

    fn leaf_key(path: &[u8]) -> Hash { hash(path) }

    pub fn get(&self, path: &[u8]) -> Option<Hash> {
        let k = Self::leaf_key(path);
        self.store.get(&k).map(|(l, _)| *l)
    }

    pub fn set(&mut self, path: &[u8], value: Hash) {
        let k = Self::leaf_key(path);
        self.store.insert(k, (value, [0; 32]));
        self.recalculate_root();
    }

    fn recalculate_root(&mut self) {
        let mut keys: Vec<_> = self.store.keys().cloned().collect();
        if keys.is_empty() {
            self.root = [0; 32];
            return;
        }
        keys.sort_unstable();
        let mut layer: Vec<Hash> = keys.iter().map(|k| self.store[k].0).collect();
        while layer.len() > 1 {
            let mut next = Vec::with_capacity((layer.len() + 1) / 2);
            for chunk in layer.chunks(2) {
                let mut hasher = blake3::Hasher::new();
                hasher.update(&chunk[0]);
                if chunk.len() == 2 {
                    hasher.update(&chunk[1]);
                }
                next.push(*hasher.finalize().as_bytes());
            }
            layer = next;
        }
        self.root = layer[0];
    }
}
