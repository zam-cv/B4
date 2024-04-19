use crate::database::Database;
use actix_web::{error, get, web, HttpResponse, Responder, Result};

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

#[get("/{user_id}/history")]
pub async fn get_player_history(
    database: web::Data<Database>,
    path: web::Path<i32>,
) -> Result<impl Responder> {
    let user_id = path.into_inner();
    let player = database
        .get_player_by_user_id(user_id)
        .await
        .map_err(|_| error::ErrorBadRequest("Failed"))?;

    if let Some(p) = player {
        if let Some(id) = p.id {
            let history = database
                .get_history_by_player_id(id)
                .await
                .map_err(|_| error::ErrorBadRequest("Failed"))?;

            println!("{:?}", history);

            Ok(HttpResponse::Ok().finish())
        } else {
            Ok(HttpResponse::NotFound().finish())
        }
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}
