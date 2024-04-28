use crate::{
    database::{Database, DbResponder},
    models,
};
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
    if crop_type.validate().is_err() {
        return Ok(HttpResponse::BadRequest().finish());
    }

    database
        .unsert_crop_types(crop_type.into_inner())
        .await
        .to_web()?;

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
    let crop_type = database.get_crop_type_by_name(name).await.to_web()?;

    Ok(HttpResponse::Ok().json(crop_type))
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The crop types were found", body = Vec<CropType>)
  )
)]
#[get("/crops")]
pub async fn get_crop_types(database: web::Data<Database>) -> Result<impl Responder> {
    let crop_types = database.get_crop_types().await.to_web()?;
    Ok(HttpResponse::Ok().json(crop_types))
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The crop type was updated")
  ),
  request_body = String
)]
#[put("/crops/{name}/description")]
pub async fn update_crop_type_description(
    database: web::Data<Database>,
    path: web::Path<String>,
    req_body: String,
) -> Result<impl Responder> {
    let name = path.into_inner();

    database
        .update_crop_type_description(name, req_body)
        .await
        .to_web()?;

    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The crop type was updated")
  ),
  request_body = String
)]
#[put("/crops/{name}/duration")]
pub async fn update_crop_type_duration(
    database: web::Data<Database>,
    path: web::Path<String>,
    req_body: String,
) -> Result<impl Responder> {
    let name = path.into_inner();
    let duration = req_body
        .parse::<i32>()
        .map_err(|_| error::ErrorBadRequest("Failed"))?;

    database
        .update_crop_type_duration(name, duration)
        .await
        .to_web()?;

    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The crop type was updated")
  ),
  request_body = String
)]
#[put("/crops/{name}/price")]
pub async fn update_crop_type_price(
    database: web::Data<Database>,
    path: web::Path<String>,
    req_body: String,
) -> Result<impl Responder> {
    let name = path.into_inner();
    let price = req_body
        .parse::<i32>()
        .map_err(|_| error::ErrorBadRequest("Failed"))?;

    database
        .update_crop_type_price(name, price)
        .await
        .to_web()?;

    Ok(HttpResponse::Ok().finish())
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

    let id = database.create_tip(tip.into_inner()).await.to_web()?;
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
    let tips = database.get_tips().await.to_web()?;
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
    if tip.validate().is_err() {
        return Ok(HttpResponse::BadRequest().finish());
    }

    let id = path.into_inner();
    tip.id = Some(id);

    database.update_tip(tip.into_inner()).await.to_web()?;
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
    database.delete_tip_by_id(id).await.to_web()?;

    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The event was created", body = vec<Event>)
  ),
)]
#[get("/events")]
pub async fn get_events(database: web::Data<Database>) -> Result<impl Responder> {
    let events = database.get_events().await.to_web()?;
    Ok(HttpResponse::Ok().json(events))
}
