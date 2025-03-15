use sha2::{Digest, Sha256};

pub fn hash_url(url: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(url);
    let result = hasher.finalize();

    let hash_bytes = &result[..8];
    let hash_value = u64::from_be_bytes(hash_bytes.try_into().expect("failed converting bytes"));
    base62::encode(hash_value)
}

pub fn generate_user_session() -> uuid::Uuid {
    uuid::Uuid::new_v4()
}