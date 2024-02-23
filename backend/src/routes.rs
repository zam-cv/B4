use crate::utils::{create_token, decode_token};
use actix_web::{post, web, Responder, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct AdminLoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct Credentials {
    token: String,
}

#[post("/login")]
async fn login_admin(info: web::Json<AdminLoginRequest>) -> Result<impl Responder> {
    let argon2 = Argon2::default();

    // TODO: get hash from database
    let sample = "$argon2id$v=19$m=19456,t=2,p=1$wX/0BU0eE/qgb8nsa6JVXA$/cDPB3Uk1niHGOmrou3zLuwWXEPdJGUCe2Ti2JMJTGo";

    if let Ok(hash) = PasswordHash::new(sample) {
        if argon2
            .verify_password(info.password.as_bytes(), &hash)
            .is_ok()
        {
            // TODO: get id from database
            if let Ok(token) = create_token(0) {
                return Ok(web::Json(Credentials { token }));
            }
        }
    }

    Err(actix_web::error::ErrorBadRequest("Failed"))
}

#[post("/register")]
async fn register_admin(info: web::Json<AdminLoginRequest>) -> Result<impl Responder> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    if let Ok(hash) = argon2.hash_password(info.password.as_bytes(), &salt) {
        // TODO: save username and hash to database and get id
        if let Ok(token) = create_token(0) {
            return Ok(web::Json(Credentials { token }));
        }
    }

    Err(actix_web::error::ErrorBadRequest("Failed"))
}
