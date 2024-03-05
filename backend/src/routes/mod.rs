use serde::{Deserialize, Serialize};

pub mod admin;
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

macro_rules! login {
  ($table:ident, $secret_key:expr) => {
      #[post("/login")]
      pub async fn login(
          database: web::Data<Database>,
          info: web::Json<models::Admin>
      ) -> impl Responder {
          if let Ok(Some(user)) = database.$table(info.username.clone()).await {
              if let Ok(password) = PasswordHash::new(&user.password) {
                  if Argon2::default()
                      .verify_password(info.password.as_bytes(), &password)
                      .is_ok()
                  {
                      if let Some(id) = user.id {
                          if let Ok(token) = utils::create_token(&$secret_key, id) {
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

pub(crate) use login;