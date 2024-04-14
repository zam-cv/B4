use crate::database::Database;
use actix_web::{error, get, web, HttpMessage, HttpResponse, HttpRequest, Responder, Result};

const CONTEXT_PATH: &str = "/api/admin/permissions";

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The permissions were found", body = Vec<PermissionType>),
    (status = 401, description = "The permissions were not found")
  )
)]
#[get("")]
pub async fn get_permissions(
    req: HttpRequest,
    database: web::Data<Database>,
) -> Result<impl Responder> {
    if let Some(id) = req.extensions().get::<i32>() {
        let permissions = database
            .get_permissions_by_admin_id(*id)
            .await
            .map_err(|_| error::ErrorBadRequest("Failed"))?;
        
        return Ok(HttpResponse::Ok().json(permissions));
    }

    Ok(HttpResponse::Unauthorized().finish())
}
