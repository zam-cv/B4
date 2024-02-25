use crate::config::{self, CONFIG};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: usize,
    pub exp: usize,
}

pub fn create_token(id: usize) -> anyhow::Result<String> {
    let my_claims = Claims {
        id,
        exp: config::TOKEN_EXPIRATION_TIME + chrono::Utc::now().timestamp() as usize,
    };

    Ok(encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(CONFIG.secret_key.as_ref()),
    )?)
}

pub fn decode_token(token: &str) -> anyhow::Result<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(CONFIG.secret_key.as_ref()),
        &Validation::new(Algorithm::HS256),
    )?;

    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_token() {
        let token = create_token(0).unwrap();
        let decoded = decode_token(&token).unwrap();

        assert_eq!(decoded.id, 0);
    }
}