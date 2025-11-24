#![recursion_limit = "512"]
pub mod fractal;
pub mod dag;
pub mod crypto;
pub mod vm;
pub mod net;
pub mod chain;
pub mod token;
pub mod light_clients;
pub mod consensus;

pub use crypto::{Hash, HASH_LEN};
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
