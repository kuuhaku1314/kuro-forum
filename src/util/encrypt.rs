use hex::encode;
use md5::{Digest, Md5};

pub fn encrypt_md5(password: &str, salt: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(password.as_bytes());
    hasher.update(salt.as_bytes());
    encode(hasher.finalize())
}
