use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

type HmacSha256 = Hmac<Sha256>;

pub fn create_sha256_from_string(s: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());
    hasher.finalize().as_slice().to_vec()
}

pub fn create_hmac256(secret: &[u8], message: &[u8]) -> Vec<u8> {
    let mut mac = HmacSha256::new_from_slice(secret).expect("HMAC can take key of any size");
    mac.update(message);
    mac.finalize().into_bytes().to_vec()
}
