use hmac::{Hmac, Mac};
use rand::distributions::{Alphanumeric, DistString};
use sha2::Sha256;
pub struct PasswordHash {
    password_hash: String,
    salt: String,
}

impl PasswordHash {
    pub fn new(password_hash: &str, salt: &str) -> PasswordHash {
        PasswordHash {
            password_hash: password_hash.to_string(),
            salt: salt.to_string(),
        }
    }
    pub fn create_hash(password: &str) -> PasswordHash {
        let salt = Alphanumeric.sample_string(&mut rand::thread_rng(), 24);

        let mut mac: Hmac<Sha256> = Hmac::new_from_slice(salt.as_bytes()).unwrap();

        mac.update(password.as_bytes());

        let hash_bytes = mac.finalize().into_bytes().to_vec();

        let password_hash = hex::encode(hash_bytes);

        PasswordHash {
            salt,
            password_hash,
        }
    }

    pub fn verify_password(&self, password_attemp: &str) -> bool {
        let mut mac: Hmac<Sha256> = Hmac::new_from_slice(&self.salt.as_bytes()).unwrap();

        mac.update(password_attemp.as_bytes());

        let hash_bytes = mac.finalize().into_bytes().to_vec();

        let attempt_hash = hex::encode(hash_bytes);

        attempt_hash.eq(&self.password_hash)
    }
}
