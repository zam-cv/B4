use crate::database::Database;
use actix_web::{error, get, web, Responder, Result};

const CONTEXT_PATH: &str = "/api/admin/player";

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The player was found", body = Player)
  ),
  params(
    ("id" = u64, Path, description = "The id of the player")
  )
)]
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
