use crate::schema;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub mod roles;
pub mod types;
pub mod users;
pub mod permissions;

pub use roles::*;
pub use permissions::*;
pub use types::*;
pub use users::*;

#[derive(Clone, Deserialize, Queryable, Selectable, Identifiable, Insertable, AsChangeset)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Player))]
#[diesel(table_name = schema::loans)]
pub struct Loan {
    #[serde(skip_deserializing, skip_serializing)]
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub status: String,
    pub cycle: i32,
    pub amount: i32,
    pub creditor: String,
    #[serde(skip_serializing)]
    pub player_id: i32,
}

#[derive(Clone, Deserialize, Queryable, Selectable, Identifiable, Insertable, AsChangeset)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Loan))]
#[diesel(table_name = schema::insurance)]
pub struct Insurance {
    #[serde(skip_deserializing, skip_serializing)]
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub insurance_type: String,
    pub sum_assured: i32,
    #[serde(skip_serializing)]
    pub loan_id: i32,
}

#[derive(Serialize, ToSchema, Queryable, Selectable, Identifiable, Insertable, Associations)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Player))]
#[diesel(table_name = schema::statistics)]
pub struct StatisticsSample {
    #[serde(skip_deserializing, skip_serializing)]
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub cycle: i32,
    pub score: i32,
    #[serde(skip_serializing)]
    pub player_id: i32,
}

#[derive(Deserialize, Validate, ToSchema)]
#[derive(Queryable, Selectable, Identifiable, Insertable, AsChangeset)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(name))]
#[diesel(table_name = schema::crop_types)]
pub struct CropType {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(range(min = 1))]
    pub price: i32,
    #[validate(range(min = 1))]
    pub duration: i32,
}

#[derive(Clone, Serialize)]
#[derive(Queryable, Selectable, Identifiable, Insertable, Associations, AsChangeset)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Player))]
#[diesel(belongs_to(CropType))]
#[diesel(table_name = schema::plots)]
pub struct Plot {
    #[serde(skip_deserializing, skip_serializing)]
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub crop_type_id: Option<String>,
    #[serde(skip_serializing)]
    pub player_id: i32,
}

#[derive(Clone, Serialize)]
#[derive(Queryable, Selectable, Identifiable, Insertable, Associations, AsChangeset)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(created_at, user_id))]
#[diesel(belongs_to(User))]
#[diesel(table_name = schema::sessions)]
pub struct Session {
    pub created_at: chrono::NaiveDateTime,
    pub user_id: i32,
    pub times: i32,
}

impl Session {
    pub fn new(user_id: i32) -> Self {
        Self {
            created_at: chrono::Utc::now().naive_utc().date().into(),
            user_id,
            times: 1,
        }
    }
}