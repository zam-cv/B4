use crate::{
    database::{Database, DbResponder},
    models::{Gender, UserType},
};
use actix_web::{get, web, HttpResponse, Responder, Result};
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
    let users = database.get_users().await.to_web()?;
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
    let user_types = database.get_user_count_by_type().await.to_web()?;
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
    let user_types = database.get_user_count_by_gender().await.to_web()?;
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
        .to_web()?;

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
    let user_types = database.get_user_locations_by_type().await.to_web()?;
    Ok(HttpResponse::Ok().json(user_types))
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The average age was found", body = f64)
  )
)]
#[get("/average-age")]
pub async fn get_average_age(database: web::Data<Database>) -> Result<impl Responder> {
    let average_age = database.get_average_age().await.to_web()?;
    Ok(HttpResponse::Ok().body(average_age.unwrap_or(0.0).to_string()))
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The average sessions were found", body = Vec<(i32, Option<f64>)>)
  )
)]
#[get("/average-sessions")]
pub async fn get_average_sessions(database: web::Data<Database>) -> Result<impl Responder> {
    let average_sessions = database
        .get_average_sessions_by_day_of_week()
        .await
        .to_web()?;

    Ok(HttpResponse::Ok().json(average_sessions))
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The average time in game was found", body = Vec<(UserType, Option<f64>)>)
  )
)]
#[get("/average-time-in-game")]
pub async fn get_average_time_in_game(database: web::Data<Database>) -> Result<impl Responder> {
    let average_time_in_game = database
        .get_average_time_in_game_by_user_type()
        .await
        .to_web()?;

    Ok(HttpResponse::Ok().json(average_time_in_game))
}
