use crate::config;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,
    pub exp: usize,
}

macro_rules! get_hash {
    ($password:expr) => {
        Argon2::default().hash_password($password.as_bytes(), &SaltString::generate(&mut OsRng))
    };
}

pub(crate) use get_hash;

pub fn create_token(secret_key: &String, id: i32) -> anyhow::Result<String> {
    let my_claims = Claims {
        id,
        exp: config::TOKEN_EXPIRATION_TIME + chrono::Utc::now().timestamp() as usize,
    };

    Ok(encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )?)
}

pub fn decode_token(secret_key: &String, token: &str) -> anyhow::Result<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key.as_ref()),
        &Validation::new(Algorithm::HS256),
    )?;

    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::CONFIG;

    #[test]
    fn test_create_token() {
        let token = create_token(&CONFIG.user_secret_key, 0).unwrap();
        let decoded = decode_token(&CONFIG.user_secret_key, &token).unwrap();

        assert_eq!(decoded.id, 0);
    }
}
