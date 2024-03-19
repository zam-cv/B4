use crate::database;
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