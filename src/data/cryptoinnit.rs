extern crate crypto;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

pub fn hash_string(s: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(s);
    hasher.result_str()
}