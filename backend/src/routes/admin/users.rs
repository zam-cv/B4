use crate::{database::Database, models::UserType};
use actix_web::{error, get, web, HttpResponse, Responder, Result};
use lazy_static::lazy_static;
use strum::IntoEnumIterator;

const CONTEXT_PATH: &str = "/api/admin/users";

lazy_static! {
  static ref USER_TYPES: Vec<String> = UserType::iter().map(|x| x.to_string()).collect();
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The users were found", body = Vec<User>)
  )
)]
#[get("")]
pub async fn get_users(database: web::Data<Database>) -> Result<impl Responder> {
    let users = database
        .get_users()
        .await
        .map_err(|_| error::ErrorBadRequest("Failed"))?;

    Ok(HttpResponse::Ok().json(users))
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The user was found", body = Vec<String>),
    (status = 404, description = "The user was not found")
  )
)]
#[get("/types")]
pub async fn get_user_types() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().json(USER_TYPES.clone()))
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The user was found", body = Vec<(UserType, i64)>),
    (status = 404, description = "The user was not found")
  )
)]
#[get("/types/count")]
pub async fn get_user_count_by_type(database: web::Data<Database>) -> Result<impl Responder> {
    let user_types = database
        .get_user_count_by_type()
        .await
        .map_err(|_| error::ErrorBadRequest("Failed"))?;

    Ok(HttpResponse::Ok().json(user_types))
}