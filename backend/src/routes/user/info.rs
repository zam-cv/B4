use crate::{
    database::Database,
    routes::{Response, Status},
};
use actix_web::{error, get, web, Responder, Result};

#[get("")]
pub async fn get_users(database: web::Data<Database>) -> Result<impl Responder> {
    let users = database
        .get_users()
        .await
        .map_err(|_| error::ErrorBadRequest("Failed"))?;

    Ok(web::Json(Response {
        message: Status::Success,
        payload: Some(users),
    }))
}

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

    Ok(web::Json(Response {
        message: Status::Success,
        payload: Some(user),
    }))
}

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

    Ok(web::Json(Response {
        message: Status::Success,
        payload: Some(statistics),
    }))
}
