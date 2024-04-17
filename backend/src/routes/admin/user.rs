use crate::database::Database;
use actix_web::{error, get, web, HttpResponse, Responder, Result};

const CONTEXT_PATH: &str = "/api/admin/user";

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The user was found", body = User),
    (status = 404, description = "The user was not found")
  ),
  params(
    ("id" = u64, Path, description = "The id of the user")
  )
)]
#[get("/{id}")]
pub async fn get_user(
    database: web::Data<Database>,
    path: web::Path<i32>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let user = database
        .get_user_by_id(id)
        .await
        .map_err(|_| error::ErrorBadRequest("Failed"))?;

    match user {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The user was found", body = Vec<StatisticsSample>),
    (status = 404, description = "The user was not found")
  ),
  params(
    ("id" = u64, Path, description = "The id of the user")
  )
)]
#[get("/statistics/{id}")]
pub async fn get_user_statistics(
    database: web::Data<Database>,
    path: web::Path<i32>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let statistics = database
        .get_statistics(id)
        .await
        .map_err(|_| error::ErrorBadRequest("Failed"))?;

    Ok(HttpResponse::Ok().json(statistics))
}
