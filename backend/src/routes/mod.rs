use crate::utils;
use actix_web::{delete, HttpResponse};

pub mod admin;
pub mod player;
pub mod user;

macro_rules! signin {
    ($table:ident, $secret_key:expr) => {
        #[post("/signin")]
        pub async fn signin(
            database: web::Data<Database>,
            profile: web::Json<models::Admin>,
        ) -> impl Responder {
            if let Ok(Some(user)) = database.$table(profile.email.clone()).await {
                if let Ok(password) = PasswordHash::new(&user.password) {
                    if Argon2::default()
                        .verify_password(profile.password.as_bytes(), &password)
                        .is_ok()
                    {
                        if let Some(id) = user.id {
                            if let Ok(token) = utils::create_token(&$secret_key, id) {
                                let cookie = utils::get_cookie_with_token(&token);
                                return HttpResponse::Ok().cookie(cookie).finish();
                            }
                        }
                    }
                }
            }

            HttpResponse::Unauthorized().body("Username or password is incorrect")
        }
    };
}

pub(crate) use signin;

#[delete("/signout")]
pub async fn signout() -> HttpResponse {
    let cookie = utils::get_cookie_with_expired_token();
    HttpResponse::Ok().cookie(cookie).finish()
}
