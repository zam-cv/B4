use crate::models::Admin;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub mod admin;
pub mod auth;

#[derive(Deserialize, ToSchema, Validate)]
pub struct Credentials {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Info {
    pub token: String,
    pub admin: Admin,
}