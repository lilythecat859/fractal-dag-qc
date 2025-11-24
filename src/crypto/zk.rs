use zk_groth16::{prove, verify, Proof, ProvingKey, VerifyingKey, ProofInput};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ZkCircuit {
    pub inputs: Vec<ProofInput>,
}

impl ZkCircuit {
    pub fn prove(&self, pk: &ProvingKey) -> Proof {
        prove(pk, &self.inputs)
    }
    pub fn verify(vk: &VerifyingKey, proof: &Proof, public: &[ProofInput]) -> bool {
        verify(vk, proof, public)
    }
}
