use crate::crypto::pq::{PublicKey as PQPub, Signature as PQSig};
use secp256k1::{ecdsa::Signature, PublicKey, Secp256k1};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum SigType {
    PQ(PQSig),
    Secp(Signature),
}

pub fn verify_sig(pk_bytes: &[u8], msg: &[u8], sig: &SigType) -> bool {
    match sig {
        SigType::PQ(s) => {
            let pk = PQPub::from_bytes(pk_bytes).unwrap();
            crate::crypto::pq::PQKeypair::verify(&pk, msg, s)
        }
        SigType::Secp(s) => {
            let pk = PublicKey::from_slice(pk_bytes).unwrap();
            Secp256k1::verification_only().verify_ecdsa(msg, s, &pk).is_ok()
        }
    }
}
