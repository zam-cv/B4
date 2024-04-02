use crate::database::Database;
use actix_web::{error, get, web, HttpResponse, Responder, Result};

const CONTEXT_PATH: &str = "/api/admin/users";

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
