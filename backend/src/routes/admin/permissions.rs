use crate::{
    database::{Database, DbResponder},
    models,
};
use actix_web::{
    delete, get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder, Result,
};
use serde::Deserialize;
use utoipa::ToSchema;

const CONTEXT_PATH: &str = "/api/admin/permissions";

#[derive(Deserialize, ToSchema)]
pub struct PermissionPayload {
    pub id: i32,
    pub permission: models::PermissionType,
}

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
        let permissions = database.get_permissions_by_admin_id(*id).await.to_web()?;
        return Ok(HttpResponse::Ok().json(permissions));
    }

    Ok(HttpResponse::Unauthorized().finish())
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The permissions were found", body = Vec<PermissionType>),
    (status = 401, description = "The permissions were not found")
  ),
  params(
    ("id" = u64, Path, description = "The admin ID")
  )
)]
#[get("/{id}")]
pub async fn get_permissions_by_admin_id(
    database: web::Data<Database>,
    path: web::Path<i32>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let permissions = database.get_permissions_by_admin_id(id).await.to_web()?;
    return Ok(HttpResponse::Ok().json(permissions));
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The permissions were found", body = Vec<PermissionType>),
    (status = 401, description = "The permissions were not found")
  ),
  params(
    ("role" = String, Path, description = "The role name")
  )
)]
#[get("/types/{role}")]
pub async fn get_permission_types(
    database: web::Data<Database>,
    path: web::Path<String>,
) -> Result<impl Responder> {
    let role = path.into_inner();
    let permissions = database.get_permissions_by_role_id(role).await.to_web()?;
    Ok(HttpResponse::Ok().json(permissions))
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "Successfully added permission"),
    (status = 401, description = "Failed to add permission")
  ),
  request_body = PermissionPayload
)]
#[post("")]
pub async fn add_permission(
    database: web::Data<Database>,
    body: web::Json<PermissionPayload>,
) -> Result<impl Responder> {
    database
        .unsert_permission_by_admin_id(body.id, body.permission)
        .await
        .to_web()?;

    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "Successfully deleted permission"),
    (status = 401, description = "Failed to delete permission")
  ),
  params(
    ("id" = i32, Path, description = "The admin ID"),
    ("permission" = PermissionType, Path, description = "The permission type") 
  )
)]
#[delete("/{id}/{permission}")]
pub async fn delete_permission(
    database: web::Data<Database>,
    path: web::Path<(i32, models::PermissionType)>,
) -> Result<impl Responder> {
    let (id, permission) = path.into_inner();

    database
        .delete_permission_by_admin_id(id, permission)
        .await
        .to_web()?;

    Ok(HttpResponse::Ok().finish())
}
