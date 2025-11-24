use nalgebra::Point3;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct LatticeNode {
    pub coord: Point3<f64>,
    pub shard: ShardId,
}

impl LatticeNode {
    pub fn distance(&self, other: &Self) -> f64 {
        nalgebra::distance(&self.coord, &other.coord)
    }
}
