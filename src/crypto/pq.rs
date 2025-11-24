use pqc_dilithium::{Keypair, PublicKey, SecretKey, Signature};
use pqc_kyber::{decapsulate, encapsulate, keypair, PublicKey as KPub, SecretKey as KSec, SharedSecret};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use crate::crypto::Hash;

#[derive(Clone, Serialize, Deserialize)]
pub struct PQKeypair {
    pub sig_pk: PublicKey,
    pub sig_sk: SecretKey,
    pub kem_pk: KPub,
    pub kem_sk: KSec,
}

impl PQKeypair {
    pub fn generate() -> Self {
        let (sig_pk, sig_sk) = Keypair::generate(&mut OsRng).into();
        let (kem_pk, kem_sk) = keypair(&mut OsRng);
        Self { sig_pk, sig_sk, kem_pk, kem_sk }
    }
    pub fn sign(&self, msg: &[u8]) -> Signature {
        Keypair::from((self.sig_pk, self.sig_sk)).sign(msg)
    }
    pub fn verify(pk: &PublicKey, msg: &[u8], sig: &Signature) -> bool {
        Keypair::verify(msg, sig, pk).is_ok()
    }
    pub fn kem_encrypt(&self, pk: &KPub) -> (SharedSecret, Vec<u8>) {
        encapsulate(pk, &mut OsRng)
    }
    pub fn kem_decrypt(&self, ct: &[u8]) -> Result<SharedSecret, pqc_kyber::KyberError> {
        decapsulate(ct, &self.kem_sk)
    }
}
