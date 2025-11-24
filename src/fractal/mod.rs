pub mod shard;
pub mod lattice;
pub mod expander;
use nalgebra::{DMatrix, DVector};
use serde::{Deserialize, Serialize};
pub type ShardId = u64;
pub const FRACTAL_DEPTH: usize = 7;
pub const FRACTAL_FANOUT: usize = 4;
