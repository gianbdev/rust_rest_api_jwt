use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header};
use log::debug;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;

use crate::models::user::LoginInfoDTO;

pub fn read_key_from_file() -> Result<Vec<u8>, std::io::Error> {
    let path = Path::new("secret.key"); // Asegúrate de que la ruta es correcta
    let key = fs::read(path)?;
    if key.len() == 16 {
        Ok(key)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Key file must be exactly 16 bytes long",
        ))
    }
}

static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    pub iat: i64,
    pub exp: i64,
    pub user: String,
    pub login_session: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenBodyResponse {
    pub token: String,
    pub token_type: String,
}

impl UserToken {
    pub fn generate_token(login: &LoginInfoDTO) -> String {
        dotenv::dotenv().expect("Failed to read .env file");
        let max_age: i64 = match env::var("MAX_AGE") {
            Ok(val) => val.parse::<i64>().unwrap_or(ONE_WEEK),
            Err(_) => ONE_WEEK,
        };

        debug!("Token Max Age: {}", max_age);

        let now = Utc::now().timestamp_nanos_opt().unwrap_or(0) / 1_000_000_000; // nanosecond -> second
        let payload = UserToken {
            iat: now,
            exp: now + max_age,
            user: login.username.clone(),
            login_session: login.login_session.clone(),
        };

        // Usa la clave leída del archivo
        let key = read_key_from_file().expect("Failed to read key from file");

        jsonwebtoken::encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(&key),
        )
        .unwrap()
    }
}
