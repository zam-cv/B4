use crate::database::Database;
use actix_web::{error, get, web, Responder, Result};

#[get("/{id}")]
pub async fn get_player(
    database: web::Data<Database>,
    path: web::Path<i32>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let player = database
        .get_player_by_id(id)
        .await
        .map_err(|_| error::ErrorBadRequest("Failed"))?;

    Ok(web::Json(player))
}

#[get("/count")]
pub async fn get_players_count(database: web::Data<Database>) -> Result<impl Responder> {
    let count = database
        .get_players_count()
        .await
        .map_err(|_| error::ErrorBadRequest("Failed"))?;

    Ok(web::Json(count))
}
