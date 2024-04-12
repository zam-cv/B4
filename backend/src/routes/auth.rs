use crate::{config::CONFIG, database::Database, models, utils, routes};
use actix_web::{delete, error, post, get, web, HttpRequest, HttpResponse, HttpMessage, Responder, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use ipinfo::{IpInfo, IpInfoConfig};
use std::sync::{Arc, Mutex};
use validator::Validate;

const CONTEXT_PATH: &str = "/api/auth";

#[utoipa::path(
    context_path = CONTEXT_PATH,
    responses(
      (status = 200, description = "The admin was found"),
      (status = 401, description = "The admin was not found")
    )
  )]
  #[get("")]
  pub async fn auth(req: HttpRequest, database: web::Data<Database>) -> Result<impl Responder> {
      if let Some(id) = req.extensions().get::<i32>() {
          let user = database
              .get_user_by_id(*id)
              .await
              .map_err(|_| error::ErrorBadRequest("Failed"))?;
  
          return match user {
              Some(_) => Ok(HttpResponse::Ok().finish()),
              None => Ok(HttpResponse::NotFound().finish()),
          };
      }
  
      Ok(HttpResponse::Unauthorized().finish())
  }

#[utoipa::path(
    context_path = CONTEXT_PATH,
    responses(
        (status = 201, description = "The credentials were correct"),
        (status = 401, description = "The credentials were incorrect")
    ),
    request_body = UserCredentials
)]
#[post("/signin")]
pub async fn signin(
    database: web::Data<Database>,
    profile: web::Json<routes::UserCredentials>,
) -> impl Responder {
    if let Ok(Some(user)) = database.get_user_by_username(profile.username.clone()).await {
        if let Ok(password) = PasswordHash::new(&user.password) {
            if Argon2::default()
                .verify_password(profile.password.as_bytes(), &password)
                .is_ok()
            {
                if let Some(id) = user.id {
                    if let Ok(token) = utils::create_token(&CONFIG.user_secret_key, id) {
                        let cookie = utils::get_cookie_with_token(&token);
                        return HttpResponse::Ok().cookie(cookie).body(token);
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
        (status = 200, description = "The user was created"),
        (status = 401, description = "The email already exists")
    ),
    request_body = User
)]
#[post("/register")]
pub async fn register(
    req: HttpRequest,
    database: web::Data<Database>,
    location_db: web::Data<Option<Arc<Mutex<ip2location::DB>>>>,
    mut user: web::Json<models::User>,
) -> Result<impl Responder> {
    if let Err(_) = user.validate() {
        return Ok(HttpResponse::Unauthorized().body("Invalid"));
    }

    if let Ok(hash) = utils::get_hash!(user.password) {
        if let Ok(None) = database.get_user_by_email(user.email.clone()).await {
            let ip = req.peer_addr().map(|addr| addr.ip());
            log::debug!("IP: {:?}", ip);

            let os = req
                .headers()
                .get("user-agent")
                .map(|ua| ua.to_str().ok())
                .flatten()
                .map(|user_agent| utils::get_os(user_agent))
                .flatten();

            let player_id = database
                .create_player()
                .await
                .map_err(|_| error::ErrorBadRequest("Failed"))?;

            user.id = None;
            user.password = hash.to_string();
            user.os = os;
            user.player_id = player_id;

            let config = IpInfoConfig {
                token: Some(CONFIG.ipinfo_token.clone()),
                ..Default::default()
            };

            if let Some(ip) = ip {
                let accepted = if let Ok(mut ipinfo) = IpInfo::new(config) {
                    let res = ipinfo.lookup(ip.to_string().as_str()).await;

                    match res {
                        Ok(r) => {
                            let mut parts = r.loc.split(',').map(|part| part.parse::<f64>());
                            if let (Some(Ok(longitud)), Some(Ok(latitud))) =
                                (parts.next(), parts.next())
                            {
                                user.latitude = Some(latitud);
                                user.longitude = Some(longitud);
                            }

                            true
                        }
                        Err(_) => false,
                    }
                } else {
                    false
                };

                // If the IP lookup fails, use the location database
                if !accepted {
                    if let Some(location_db) = location_db.get_ref() {
                        if let Ok(mut location_db) = location_db.lock() {
                            if let Ok(ip2location::Record::LocationDb(rec)) = location_db.ip_lookup(ip)
                            {
                                user.latitude = rec.latitude.map(|lat| lat as f64);
                                user.longitude = rec.longitude.map(|long| long as f64);
                            }
                        }
                    }
                }
            }

            let user_id = database
                .create_user(user.into_inner())
                .await
                .map_err(|_| error::ErrorBadRequest("Failed"))?;

            if let Ok(token) = utils::create_token(&CONFIG.user_secret_key, user_id) {
                let cookie = utils::get_cookie_with_token(&token);
                return Ok(HttpResponse::Ok().cookie(cookie).finish());
            }
        }

        return Ok(HttpResponse::Unauthorized().body("Email already exists"));
    }

    Err(actix_web::error::ErrorBadRequest("Failed"))
}

#[utoipa::path(
    context_path = CONTEXT_PATH,
    responses((status = 200, description = "The user was signed out"))
)]
#[delete("/signout")]
pub async fn signout() -> HttpResponse {
    let cookie = utils::get_cookie_with_expired_token();
    HttpResponse::Ok().cookie(cookie).finish()
}
