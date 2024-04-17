use crate::models;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub mod admin;
pub mod auth;

#[derive(Deserialize, ToSchema)]
pub struct UserCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct AdminCredentials {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AdminInfo {
    pub token: String,
    pub admin: models::Admin,
}
