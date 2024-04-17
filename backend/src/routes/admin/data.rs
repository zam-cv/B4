use crate::{database::Database, models};
use actix_web::{delete, error, get, post, put, web, HttpResponse, Responder, Result};
use validator::Validate;

const CONTEXT_PATH: &str = "/api/admin/data";

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The crop type was created")
  ),
  request_body = CropType
)]
#[post("/crops")]
pub async fn create_crop_type(
    database: web::Data<Database>,
    crop_type: web::Json<models::CropType>,
) -> Result<impl Responder> {
    database
        .unsert_crop_types(crop_type.into_inner())
        .await
        .map_err(|_| error::ErrorBadRequest("Failed"))?;

    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The crop types were found", body = Vec<CropType>)
  )
)]
#[get("/crops/{name}")]
pub async fn get_crop_type(
    database: web::Data<Database>,
    path: web::Path<String>,
) -> Result<impl Responder> {
    let name = path.into_inner();
    let crop_type = database
        .get_crop_type_by_name(name)
        .await
        .map_err(|_| error::ErrorBadRequest("Failed"))?;

    Ok(HttpResponse::Ok().json(crop_type))
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The tip was created", body = String)
  ),
  request_body = Tip
)]
#[post("/tips")]
pub async fn create_tip(
    database: web::Data<Database>,
    tip: web::Json<models::Tip>,
) -> Result<impl Responder> {
    if tip.validate().is_err() {
        return Ok(HttpResponse::BadRequest().finish());
    }

    let id = database
        .create_tip(tip.into_inner())
        .await
        .map_err(|_| error::ErrorBadRequest("Failed"))?;

    Ok(HttpResponse::Ok().body(id.to_string()))
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The tips were found", body = Vec<Tip>)
  )
)]
#[get("/tips")]
pub async fn get_tips(database: web::Data<Database>) -> Result<impl Responder> {
    let tips = database
        .get_tips()
        .await
        .map_err(|_| error::ErrorBadRequest("Failed"))?;

    Ok(HttpResponse::Ok().json(tips))
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The tip was updated")
  ),
  request_body = Tip
)]
#[put("/tips/{id}")]
pub async fn update_tip(
    database: web::Data<Database>,
    path: web::Path<i32>,
    mut tip: web::Json<models::Tip>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    tip.id = Some(id);

    database
        .update_tip(tip.into_inner())
        .await
        .map_err(|_| error::ErrorBadRequest("Failed"))?;

    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The tip was deleted")
  )
)]
#[delete("/tips/{id}")]
pub async fn delete_tip(
    database: web::Data<Database>,
    path: web::Path<i32>,
) -> Result<impl Responder> {
    let id = path.into_inner();

    database
        .delete_tip_by_id(id)
        .await
        .map_err(|_| error::ErrorBadRequest("Failed"))?;

    Ok(HttpResponse::Ok().finish())
}
