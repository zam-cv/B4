use crate::{
    config::CONFIG,
    database::Database,
    models,
    routes::{login, Credentials, Response, Status},
    utils,
};
use actix_web::{get, post, web, Responder, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};

login!(get_admin_by_username, CONFIG.admin_secret_key);

#[post("/register")]
pub async fn register(
    database: web::Data<Database>,
    info: web::Json<models::Admin>,
) -> Result<impl Responder> {
    if let Ok(hash) = utils::get_hash!(info.password) {
        if let Ok(None) = database.get_admin_by_username(info.username.clone()).await {
            let id = database
                .create_admin(models::Admin {
                    id: None,
                    username: info.username.clone(),
                    password: hash.to_string(),
                })
                .await?;

            if let Ok(token) = utils::create_token(&CONFIG.admin_secret_key, id) {
                return Ok(web::Json(Response {
                    message: Status::Success,
                    payload: Some(Credentials { token }),
                }));
            }
        }

        return Ok(web::Json(Response {
            message: Status::Incorrect("Username already exists"),
            payload: None,
        }));
    }

    Err(actix_web::error::ErrorBadRequest("Failed"))
}

#[get("/users")]
pub async fn get_users(database: web::Data<Database>) -> Result<impl Responder> {
    let users = database.get_users().await?;

    Ok(web::Json(Response {
        message: Status::Success,
        payload: Some(users),
    }))
}

#[get("/statistics/{id}")]
pub async fn get_statistics(
    database: web::Data<Database>,
    path: web::Path<i32>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let statistics = database.get_statistics(id).await?;

    Ok(web::Json(Response {
        message: Status::Success,
        payload: Some(statistics),
    }))
}
