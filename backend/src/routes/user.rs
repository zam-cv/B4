use crate::{
    config::{self, CONFIG},
    database::Database,
    models,
    routes::{login, Credentials, Response, Status},
    utils,
};
use actix_web::{post, web, Responder, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};

login!(get_user_by_username, CONFIG.user_secret_key);

#[post("/register")]
pub async fn register(
    database: web::Data<Database>,
    info: web::Json<models::User>,
) -> Result<impl Responder> {
    if let Ok(hash) = utils::get_hash!(info.password) {
        if let Ok(None) = database.get_user_by_username(info.username.clone()).await {
            let id = database
                .create_user(models::User {
                    id: None,
                    username: info.username.clone(),
                    password: hash.to_string(),
                    balance_cash: config::INITIAL_BALANCE_CASH,
                    balance_verqor: config::INITIAL_BALANCE_VERQOR,
                    balance_coyote: config::INITIAL_BALANCE_COYOTE,
                    current_day: chrono::Local::now().naive_local(),
                })
                .await?;

            if let Ok(token) = utils::create_token(&CONFIG.user_secret_key, id) {
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
