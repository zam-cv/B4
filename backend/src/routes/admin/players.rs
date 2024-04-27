use crate::{database, config};
use actix_web::{error, get, web, Responder, Result};

const CONTEXT_PATH: &str = "/api/admin/players";

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The players were found", body = u64)
  )
)]
#[get("/count")]
pub async fn get_players_count(database: web::Data<database::Database>) -> Result<impl Responder> {
    let count = database
        .get_players_count()
        .await
        .map_err(|_| error::ErrorBadRequest("Failed"))?;

    Ok(web::Json(count))
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The average time was found", body = i32)
  )
)]
#[get("/average-time")]
pub async fn get_average_time_in_game(database: web::Data<database::Database>) -> Result<impl Responder> {
    let average_time = database
        .get_average_time_in_game()
        .await
        .map_err(|_| error::ErrorBadRequest("Failed"))?;

    Ok(web::Json(average_time))
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The top players were found", body = Vec<String>)
  )
)]
#[get("/top-players")]
pub async fn get_top_players(database: web::Data<database::Database>) -> Result<impl Responder> {
    let top_players = database
        .get_top_players(config::TOP_PLAYERS)
        .await
        .map_err(|_| error::ErrorBadRequest("Failed"))?;

    Ok(web::Json(top_players))
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The average money was found", body = Vec<(MoneyType, f64)>)
  )
)]
#[get("/average-money")]
pub async fn get_average_money(database: web::Data<database::Database>) -> Result<impl Responder> {
    let average_money = database
        .get_average_money()
        .await
        .map_err(|_| error::ErrorBadRequest("Failed"))?;

    Ok(web::Json(average_money))
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The average score was found", body = f64)
  )
)]
#[get("/average-score")]
pub async fn get_average_score(database: web::Data<database::Database>) -> Result<impl Responder> {
    let average_score = database
        .get_average_score()
        .await
        .map_err(|_| error::ErrorBadRequest("Failed"))?;

    Ok(web::Json(average_score))
}