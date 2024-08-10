use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).unwrap()
}

pub async fn verify_password(password: &str, hash: &str) -> bool {
    verify(password, hash).unwrap()
}

pub fn create_jwt(username: &str) -> String {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let claims = Claims {
        sub: username.to_owned(),
        exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
    };
    let encoding_key = EncodingKey::from_secret(secret.as_bytes());
    encode(&Header::default(), &claims, &encoding_key).unwrap()
}

pub fn validate_jwt(token: &str) -> bool {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    decode::<Claims>(token, &decoding_key, &Validation::default()).is_ok()
}
