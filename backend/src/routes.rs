use crate::{database::Database, models::*, utils::create_token};
use actix_web::{post, web, Responder, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Response<'a, T> {
    message: &'a str,
    payload: Option<T>,
}

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
async fn login_admin(
    database: web::Data<Database>,
    info: web::Json<AdminLoginRequest>,
) -> Result<impl Responder> {
    let argon2 = Argon2::default();

    if let Ok(Some(admin)) = database.get_admin_by_username(info.username.clone()).await {
        if let Ok(password) = PasswordHash::new(&admin.password) {
            if argon2
                .verify_password(info.password.as_bytes(), &password)
                .is_ok()
            {
                if let Ok(token) = create_token(admin.id as usize) {
                    return Ok(web::Json(Response {
                        message: "Success",
                        payload: Some(Credentials { token }),
                    }));
                }
            }
        }

        return Ok(web::Json(Response {
            message: "Username or password is incorrect",
            payload: None,
        }));
    }

    Err(actix_web::error::ErrorBadRequest("Failed"))
}

#[post("/register")]
async fn register_admin(
    database: web::Data<Database>,
    info: web::Json<AdminLoginRequest>,
) -> Result<impl Responder> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    if let Ok(hash) = argon2.hash_password(info.password.as_bytes(), &salt) {
        if let Ok(None) = database.get_admin_by_username(info.username.clone()).await {
            let id = database
                .create_admin(NewAdmin {
                    username: info.username.clone(),
                    password: hash.to_string(),
                })
                .await?;

            if let Ok(token) = create_token(id) {
                return Ok(web::Json(Response {
                    message: "Success",
                    payload: Some(Credentials { token }),
                }));
            }
        } else {
            return Ok(web::Json(Response {
                message: "Username already exists",
                payload: None,
            }));
        }
    }

    Err(actix_web::error::ErrorBadRequest("Failed"))
}
