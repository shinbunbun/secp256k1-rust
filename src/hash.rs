use sha2::{Digest, Sha256};

pub fn create_sha256_from_string(s: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());
    hasher.finalize().as_slice().to_vec()
}
