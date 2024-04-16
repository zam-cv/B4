use crate::{schema, utils, models::types::*, config};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Validate, ToSchema, Clone)]
#[derive(Queryable, Selectable, Identifiable, Insertable)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(id))]
#[diesel(table_name = schema::admins)]
pub struct Admin {
    #[serde(skip_deserializing)]
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    #[validate(email)]
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    #[serde(skip_deserializing)]
    pub role_id: String,
}

#[derive(Clone, Deserialize, Serialize, Validate, Debug, PartialEq, ToSchema)]
#[derive(Queryable, Selectable, Identifiable, Insertable, AsChangeset)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(id))]
#[diesel(table_name = schema::users)]
pub struct User {
    #[serde(skip_deserializing)]
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub user_type: UserType,
    #[validate(length(min = 1, max = 20))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[serde(skip_serializing_if = "utils::always_skip")]
    pub password: String,
    pub gender: Gender,
    #[serde(skip_deserializing)]
    #[validate(length(max = 15))]
    pub os: Option<String>,
    #[serde(skip_deserializing, skip_serializing)]
    pub player_id: i32,
    #[serde(skip_deserializing)]
    pub latitude: Option<f64>,
    #[serde(skip_deserializing)]
    pub longitude: Option<f64>,
    #[validate(range(min = 1920, max = 3000))]
    pub year_of_birth: i32,
    #[serde(skip_deserializing)]
    pub role_id: String,
}

#[derive(Clone, Deserialize, Serialize, ToSchema)]
#[derive(Queryable, Selectable, Identifiable, Insertable, AsChangeset)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(id))]
#[diesel(belongs_to(User))]
#[diesel(table_name = schema::players)]
pub struct Player {
    #[serde(skip_deserializing, skip_serializing)]
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    #[serde(skip_deserializing, skip_serializing)]
    pub time_in_game: f64,
    pub current_cycle: i32,
    pub current_score: f64,
    pub balance_cash: i32,
    pub balance_verqor: i32,
    pub balance_coyote: i32,
    pub max_plots: i32,
}

impl Player {
    pub fn default() -> Self {
        Player {
            id: None,
            time_in_game: config::INITIAL_TIME,
            current_cycle: config::INITIAL_CYCLE,
            current_score: config::INITIAL_SCORE,
            balance_cash: config::INITIAL_BALANCE_CASH,
            balance_verqor: config::INITIAL_BALANCE,
            balance_coyote: config::INITIAL_BALANCE,
            max_plots: config::INITIAL_MAX_PLOTS,
        }
    }
}