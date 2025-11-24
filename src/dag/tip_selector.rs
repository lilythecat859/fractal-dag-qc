use crate::dag::{VertexStore, Vertex};
use crate::crypto::Hash;
use rand::prelude::*;

pub struct TipSelector;

impl TipSelector {
    pub fn select<V: VertexStore>(store: &V, shard: ShardId) -> Vec<Hash> {
        let candidates = store.tips(shard);
        let mut rng = thread_rng();
        candidates.into_iter().choose_multiple(&mut rng, 2)
    }
}
