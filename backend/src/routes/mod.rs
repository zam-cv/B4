use crate::utils;
use actix_web::{delete, HttpResponse};

pub mod admin;
pub mod player;
pub mod user;

#[delete("/signout")]
pub async fn signout() -> HttpResponse {
    let cookie = utils::get_cookie_with_expired_token();
    HttpResponse::Ok().cookie(cookie).finish()
}
