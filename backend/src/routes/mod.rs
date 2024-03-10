use serde::{Deserialize, Serialize};

pub mod admin;
pub mod user;
pub mod player;

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
          user: web::Json<models::Admin>
      ) -> impl Responder {
          if let Ok(Some(user)) = database.$table(user.email.clone()).await {
              if let Ok(password) = PasswordHash::new(&user.password) {
                  if Argon2::default()
                      .verify_password(user.password.as_bytes(), &password)
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

pub(crate) use signin;