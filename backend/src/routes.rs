use crate::{database::Database, models::*, utils::create_token, CONFIG};
use actix_web::{get, post, web, Responder, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub enum Status<'a> {
    Success,
    Incorrect(&'a str),
}

#[derive(Serialize)]
pub struct Response<'a, T> {
    pub message: Status<'a>,
    pub payload: Option<T>,
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

macro_rules! login {
    ($name:ident, $table:ident, $secret_key:expr) => {
        #[post("/login")]
        pub async fn $name(
            database: web::Data<Database>,
            info: web::Json<AdminLoginRequest>,
        ) -> impl Responder {
            let argon2 = Argon2::default();

            if let Ok(Some(user)) = database.$table(info.username.clone()).await {
                if let Ok(password) = PasswordHash::new(&user.password) {
                    if argon2
                        .verify_password(info.password.as_bytes(), &password)
                        .is_ok()
                    {
                        if let Some(id) = user.id {
                            if let Ok(token) = create_token(&$secret_key, id) {
                                return web::Json(Response {
                                    message: Status::Success,
                                    payload: Some(Credentials { token }),
                                });
                            }
                        }
                    }
                }
            }

            web::Json(Response {
                message: Status::Incorrect("Username or password is incorrect"),
                payload: None,
            })
        }
    };
}

macro_rules! register {
    ($name:ident, $table:ident, $create:ident, $model:ident, $secret_key:expr) => {
        #[post("/register")]
        pub async fn $name(
            database: web::Data<Database>,
            info: web::Json<AdminLoginRequest>,
        ) -> Result<impl Responder> {
            let salt = SaltString::generate(&mut OsRng);
            let argon2 = Argon2::default();

            if let Ok(hash) = argon2.hash_password(info.password.as_bytes(), &salt) {
                if let Ok(None) = database.$table(info.username.clone()).await {
                    let id = database
                        .$create($model {
                            id: None,
                            username: info.username.clone(),
                            password: hash.to_string(),
                        })
                        .await?;

                    if let Ok(token) = create_token(&$secret_key, id) {
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
    };
}

// login and register for user
login!(login_user, get_user_by_username, CONFIG.user_secret_key);
register!(
    register_user,
    get_user_by_username,
    create_user,
    User,
    CONFIG.user_secret_key
);

// login and register for admin
login!(login_admin, get_admin_by_username, CONFIG.admin_secret_key);
register!(
    register_admin,
    get_admin_by_username,
    create_admin,
    Admin,
    CONFIG.admin_secret_key
);

#[get("/users")]
pub async fn get_users(database: web::Data<Database>) -> Result<impl Responder> {
    let users = database.get_users().await?;

    Ok(web::Json(Response {
        message: Status::Success,
        payload: Some(users),
    }))
}
