use bcrypt::{DEFAULT_COST, hash, verify};

pub fn hash_password(password: &str) -> String {
    return hash(password, DEFAULT_COST).unwrap();
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    return verify(password, hash).unwrap();
}