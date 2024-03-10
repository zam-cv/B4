use crate::{
    config::CONFIG,
    database::Database,
    models,
    routes::{signin, Credentials, Response, Status},
    utils,
};
use actix_web::{error, post, web, Responder, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};

signin!(get_admin_by_email, CONFIG.admin_secret_key);

#[post("/register")]
pub async fn register(
    database: web::Data<Database>,
    admin: web::Json<models::Admin>,
) -> Result<impl Responder> {
    if let Ok(hash) = utils::get_hash!(admin.password) {
        if let Ok(None) = database.get_admin_by_email(admin.email.clone()).await {
            let id = database
                .create_admin(models::Admin {
                    id: None,
                    email: admin.email.clone(),
                    password: hash.to_string(),
                })
                .await
                .map_err(|_| error::ErrorBadRequest("Failed"))?;

            if let Ok(token) = utils::create_token(&CONFIG.admin_secret_key, id) {
                return Ok(web::Json(Response {
                    message: Status::Success,
                    payload: Some(Credentials { token }),
                }));
            }
        }

        return Ok(web::Json(Response {
            message: Status::Incorrect("Email already exists"),
            payload: None,
        }));
    }

    Err(error::ErrorBadRequest("Failed"))
}

#[post("/crops")]
pub async fn create_crop_type(
    database: web::Data<Database>,
    crop_type: web::Json<models::CropType>,
) -> Result<impl Responder> {
    database
        .unsert_crop_types(crop_type.into_inner())
        .await
        .map_err(|_| error::ErrorBadRequest("Failed"))?;

    Ok(web::Json(Response {
        message: Status::Success,
        payload: None::<()>,
    }))
}
