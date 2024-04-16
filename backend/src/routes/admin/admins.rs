use crate::database::Database;
use actix_web::{error, get, delete, web, HttpResponse, Responder, Result};

const CONTEXT_PATH: &str = "/api/admin/admins";

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The users were found", body = Vec<Admin>)
  )
)]
#[get("")]
pub async fn get_admins(database: web::Data<Database>) -> Result<impl Responder> {
    let admins = database
        .get_admins()
        .await
        .map_err(|_| error::ErrorBadRequest("Failed"))?;

    Ok(HttpResponse::Ok().json(admins))
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The admin was deleted"),
    (status = 401, description = "The admin was not found")
  ),
  request_body = u32
)]
#[delete("/{id}")]
pub async fn delete_admin(
    default_admin_id: web::Data<i32>,
    database: web::Data<Database>,
    path: web::Path<i32>,
) -> Result<impl Responder> {
    let admin_id = path.into_inner();

    if &admin_id == default_admin_id.get_ref() {
        return Ok(HttpResponse::Unauthorized().body("Cannot delete default admin"));
    }

    database
        .delete_admin_by_id(admin_id)
        .await
        .map_err(|_| error::ErrorBadRequest("Failed"))?;

    Ok(HttpResponse::Ok().finish())
}