pub mod pq;
pub mod zk;
pub mod sig;
use blake3::Hasher;
pub const HASH_LEN: usize = 32;
pub type Hash = [u8; HASH_LEN];
pub fn hash(data: &[u8]) -> Hash {
    *blake3::hash(data).as_bytes()
}
