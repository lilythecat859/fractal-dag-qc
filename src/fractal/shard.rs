use crate::fractal::{ShardId, FRACTAL_FANOUT};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Shard {
    pub id: ShardId,
    pub depth: usize,
    pub children: Vec<ShardId>,
}

impl Shard {
    pub fn root() -> Self {
        Self { id: 0, depth: 0, children: Vec::new() }
    }
    pub fn split(&self) -> Vec<Shard> {
        let base = self.id * FRACTAL_FANOUT as u64;
        (0..FRACTAL_FANOUT)
            .map(|i| Shard {
                id: base + i as u64,
                depth: self.depth + 1,
                children: Vec::new(),
            })
            .collect()
    }
}
