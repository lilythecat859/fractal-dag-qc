use crate::dag::{Vertex, VertexStore};
use crate::crypto::Hash;
use dashmap::DashMap;
use std::sync::Arc;

pub struct Dag {
    pub vertices: Arc<DashMap<Hash, Vertex>>,
}

impl Dag {
    pub fn new() -> Self {
        Self { vertices: Arc::new(DashMap::new()) }
    }
    pub fn insert(&self, v: Vertex) {
        self.vertices.insert(v.tx_hash, v);
    }
}

pub trait VertexStore {
    fn tips(&self, shard: ShardId) -> Vec<Hash>;
}
