use crate::{config::CONFIG, database::Database, models, routes::signin, utils};
use actix_web::{error, get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use validator::Validate;

signin!(get_admin_by_email, CONFIG.admin_secret_key);

#[post("/register")]
pub async fn register(
    database: web::Data<Database>,
    admin: web::Json<models::Admin>,
) -> Result<impl Responder> {
    if let Err(_) = admin.validate() {
        return Ok(HttpResponse::Unauthorized().body("Invalid"));
    }

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
                let cookie = utils::get_cookie_with_token(&token);
                return Ok(HttpResponse::Ok().cookie(cookie).finish());
            }
        }

        return Ok(HttpResponse::Unauthorized().body("Email already exists"));
    }

    Err(error::ErrorBadRequest("Failed"))
}

#[get("")]
pub async fn auth(req: HttpRequest, database: web::Data<Database>) -> Result<impl Responder> {
    if let Some(id) = req.extensions().get::<i32>() {
        let admin = database
            .get_admin_by_id(*id)
            .await
            .map_err(|_| error::ErrorBadRequest("Failed"))?;

        return match admin {
            Some(admin) => Ok(HttpResponse::Ok().json(admin)),
            None => Ok(HttpResponse::NotFound().finish()),
        };
    }

    Ok(HttpResponse::Unauthorized().finish())
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

    Ok(HttpResponse::Ok().finish())
}
