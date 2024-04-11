use crate::config;
use actix_web::cookie::{self, Cookie};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use woothee::parser::Parser;

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

pub fn get_hash_in_string(password: &str) -> anyhow::Result<String> {
    if let Ok(hash) = get_hash!(password) {
        Ok(hash.to_string())
    } else {
        Err(anyhow::anyhow!("Failed to hash the password"))
    }
}

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

/// The function `get_cookie_with_expired_token` creates a cookie with an expired token.
pub fn get_cookie_with_expired_token() -> Cookie<'static> {
    Cookie::build("token", "")
        .http_only(true)
        .secure(true)
        .same_site(cookie::SameSite::Strict)
        .path("/")
        .expires(cookie::time::OffsetDateTime::now_utc() - cookie::time::Duration::days(1))
        .finish()
}

/// The function `get_cookie_with_token` in Rust creates a cookie with a specified token value and
/// additional security settings.
pub fn get_cookie_with_token<'a>(token: &'a str) -> Cookie<'a> {
    Cookie::build("token", token)
        .http_only(true)
        .secure(true)
        .same_site(cookie::SameSite::Strict)
        .path("/")
        .finish()
}

// The function `get_os` in Rust returns the operating system of the user based on the user agent.
pub fn get_os(user_agent: &str) -> Option<String> {
    let parser = Parser::new();
    parser.parse(user_agent).map(|ua| ua.os.to_string())
}

// This function makes the field not show when responding the structure in a request
// but it does show in the documentation api
pub fn always_skip<T>(_: &T) -> bool {
    true
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
