use crate::{bank::Bank, database::{Database, DbResponder}};
use actix_web::{error, get, web, HttpResponse, Responder, Result};
use std::collections::HashMap;

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
        .to_web()?;

    Ok(web::Json(player))
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The history of the player", body = Vec<(Statistic, Vec<(String, EventType)>)>)
  ),
  params(
    ("user_id" = u64, Path, description = "The user id of the player")
  )
)]
#[get("/{user_id}/history")]
pub async fn get_player_history(
    database: web::Data<Database>,
    path: web::Path<i32>,
) -> Result<impl Responder> {
    let user_id = path.into_inner();
    let player = database
        .get_player_by_user_id(user_id)
        .await
        .to_web()?;

    if let Some(p) = player {
        if let Some(id) = p.id {
            let history = database
                .get_history_by_player_id(id)
                .await
                .map_err(|_| error::ErrorBadRequest("Failed"))?;

            let history = history
                .into_iter()
                .map(|(statistic, events)| {
                    let mut response = Vec::new();

                    for (event, functions) in events {
                        let mut variables = HashMap::new();

                        for (function, value) in functions {
                            if let Some(key) = function.key {
                                variables.insert(key, Ok(value.content));
                            }
                        }

                        let message = Bank::get_message_from_event(&event, &variables);
                        response.push((message, event.event_type));
                    }

                    (statistic, response)
                })
                .collect::<Vec<_>>();

            Ok(HttpResponse::Ok().json(history))
        } else {
            Ok(HttpResponse::NotFound().finish())
        }
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}
