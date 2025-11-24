use crate::chain::smt::SMT;
use crate::crypto::Hash;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct State {
    pub smt: SMT,
}

impl Default for State {
    fn default() -> Self { Self { smt: SMT::new() } }
}

impl State {
    pub fn apply(&mut self, vertices: &[Hash]) {
        for v in vertices {
            self.smt.set(v, [1; 32]);
        }
    }
}
