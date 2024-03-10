use serde::{Deserialize, Serialize};

pub mod admin;
pub mod player;
pub mod user;

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

#[derive(Serialize, Deserialize)]
struct Credentials {
    token: String,
}

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
                                let cookie = Cookie::build("token", &token)
                                    .http_only(true)
                                    .secure(true)
                                    .same_site(actix_web::cookie::SameSite::Strict)
                                    .path("/")
                                    .finish();
                                
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
