use ::serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, encode, errors::Error, DecodingKey, EncodingKey, Header, Validation};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
pub struct TokenClaims {
    exp: usize,
    pub type_: i32,
    pub sub: String,
    pub first_name: String,
    pub last_name: String,
    pub managed_store_id: Option<i32>
}

impl TokenClaims {
    pub fn new(type_: i32, sub: &str, first_name: &str, last_name: &str, managed_store_id: Option<i32>) -> Self {
        let mut timer = SystemTime::now();
        //7 days duration
        timer = timer + Duration::from_secs(604800);

        let exp_time = timer.duration_since(UNIX_EPOCH).unwrap().as_secs();

        Self {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            sub: sub.to_string(),
            managed_store_id, 
            type_,
            exp: exp_time as usize,
        }
    }
    pub fn sign_token(&self) -> Result<String, Error> {
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        match encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(jwt_secret.as_ref()),
        ) {
            Ok(token) => Ok(token),
            Err(e) => Err(e),
        }
    }

    pub fn decode_token(token: &str) -> Result<Self, Error> {
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        match decode::<Self>(
            &token,
            &DecodingKey::from_secret(jwt_secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(decoded) => Ok(decoded.claims),
            Err(e) => Err(e),
        }
    }
}
