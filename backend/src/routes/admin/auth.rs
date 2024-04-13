use crate::{config::CONFIG, database::Database, models, routes, utils};
use actix_web::{
    delete, error, get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder, Result,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use validator::Validate;

const CONTEXT_PATH: &str = "/api/admin/auth";

#[utoipa::path(
    context_path = CONTEXT_PATH,
    responses(
      (status = 200, description = "The admin was created"),
      (status = 401, description = "The email already exists")
    ),
    request_body = AdminCredentials
  )]
#[post("/register")]
pub async fn register(
    database: web::Data<Database>,
    admin: web::Json<routes::AdminCredentials>,
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
                return Ok(HttpResponse::Ok().cookie(cookie).body(id.to_string()));
            }
        }

        return Ok(HttpResponse::Unauthorized().body("Email already exists"));
    }

    Err(error::ErrorBadRequest("Failed"))
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The admin was found", body = Admin),
    (status = 401, description = "The admin was not found")
  )
)]
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

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The credentials were correct", body = Info),
    (status = 401, description = "The credentials were incorrect")
  ),
  request_body = Credentials
)]
#[post("/signin")]
pub async fn signin(
    database: web::Data<Database>,
    profile: web::Json<routes::AdminCredentials>,
) -> impl Responder {
    if let Ok(Some(admin)) = database.get_admin_by_email(profile.email.clone()).await {
        if let Ok(password) = PasswordHash::new(&admin.password) {
            if Argon2::default()
                .verify_password(profile.password.as_bytes(), &password)
                .is_ok()
            {
                if let Some(id) = admin.id {
                    if let Ok(token) = utils::create_token(&CONFIG.admin_secret_key, id) {
                        let cookie = utils::get_cookie_with_token(&token);
                        return HttpResponse::Ok()
                            .cookie(cookie)
                            .json(routes::AdminInfo { token, admin });
                    }
                }
            }
        }
    }

    HttpResponse::Unauthorized().body("Username or password is incorrect")
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The admin was deleted")
  )
)]
#[delete("/signout")]
pub async fn signout() -> HttpResponse {
    let cookie = utils::get_cookie_with_expired_token();
    HttpResponse::Ok().cookie(cookie).finish()
}
