use crate::{database::Database, models};
use actix_web::{error, post, get, web, HttpResponse, Responder, Result};

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