use crate::crypto::Hash;
use crate::fractal::ShardId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Vertex {
    pub tx_hash: Hash,
    pub parents: Vec<Hash>,
    pub shard: ShardId,
    pub height: u64,
    pub weight: u64,
}

impl Vertex {
    pub fn new(tx_hash: Hash, parents: Vec<Hash>, shard: ShardId) -> Self {
        Self { tx_hash, parents, shard, height: 0, weight: 1 }
    }
}
