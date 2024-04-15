use crate::{database::Database, models::{UserType, Gender}};
use actix_web::{error, get, web, HttpResponse, Responder, Result};
use lazy_static::lazy_static;
use strum::IntoEnumIterator;

const CONTEXT_PATH: &str = "/api/admin/users";
const AGE_RANGE_STEP: i32 = 10;

lazy_static! {
  static ref USER_TYPES: Vec<UserType> = UserType::iter().collect();
  static ref GENDERS: Vec<Gender> = Gender::iter().collect();
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
    (status = 200, description = "The types were found", body = Vec<UserType>),
    (status = 404, description = "The types were not found")
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

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The genders were found", body = Vec<Gender>),
    (status = 404, description = "The genders were not found")
  )
)]
#[get("/genders")]
pub async fn get_user_genders() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().json(GENDERS.clone()))
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The user was found", body = Vec<(String, i64)>),
    (status = 404, description = "The user was not found")
  )
)]
#[get("/genders/count")]
pub async fn get_user_count_by_gender(database: web::Data<Database>) -> Result<impl Responder> {
    let user_types = database
        .get_user_count_by_gender()
        .await
        .map_err(|_| error::ErrorBadRequest("Failed"))?;
    
    Ok(HttpResponse::Ok().json(user_types))
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The user was found", body = Vec<(String, i64)>),
    (status = 404, description = "The user was not found")
  )
)]
#[get("/ages/count")]
pub async fn get_user_count_by_age_range(database: web::Data<Database>) -> Result<impl Responder> {
    let user_types = database
        .get_user_count_by_age_range(AGE_RANGE_STEP)
        .await
        .map_err(|_| error::ErrorBadRequest("Failed"))?;
    
    Ok(HttpResponse::Ok().json(user_types))
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The locations were found", body = Vec<(User_Type, Vec<(f64, f64)>)>),
    (status = 404, description = "The locations were not found")
  )
)]
#[get("/locations/types")]
pub async fn get_user_locations_by_type(database: web::Data<Database>) -> Result<impl Responder> {
    let user_types = database
        .get_user_locations_by_type()
        .await
        .map_err(|_| error::ErrorBadRequest("Failed"))?;
    
    Ok(HttpResponse::Ok().json(user_types))
}