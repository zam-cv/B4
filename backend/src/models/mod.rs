use std::hash::Hash;

use crate::schema;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub mod roles;
pub mod types;
pub mod users;
pub mod permissions;
pub mod events;

pub use roles::*;
pub use permissions::*;
pub use types::*;
pub use users::*;
pub use events::*;

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

#[derive(Clone, Debug)]
#[derive(Serialize, ToSchema, Queryable, Selectable, Identifiable, Insertable, Associations)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Player))]
#[diesel(table_name = schema::statistics)]
pub struct Statistic {
    #[serde(skip_deserializing, skip_serializing)]
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub cycle: i32,
    pub score: f64,
    #[serde(skip_serializing)]
    pub player_id: i32,
}

impl Hash for Statistic {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Statistic {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
        self.cycle == other.cycle &&
        (self.score - other.score).abs() < 0.00001 &&
        self.player_id == other.player_id
    }
}

impl Eq for Statistic {}

#[derive(Deserialize, Validate, ToSchema, Serialize, Clone)]
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
    #[validate(length(min = 1, max = 500))]
    pub description: String,
}

#[derive(Clone, Serialize, Deserialize)]
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
    pub quantity: i32,
    #[serde(skip_deserializing, skip_serializing)]
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

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
#[derive(Queryable, Selectable, Identifiable, Insertable, AsChangeset)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(id))]
#[diesel(table_name = schema::tips)]
pub struct Tip {
    #[serde(skip_deserializing)]
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    #[validate(length(min = 1, max = 500))]
    pub content: String,
}

#[derive(Clone, Serialize)]
#[derive(Queryable, Selectable, Identifiable, Insertable, Associations)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(player_id, tip_id))]
#[diesel(belongs_to(Player))]
#[diesel(belongs_to(Tip))]
#[diesel(table_name = schema::player_tips)]
pub struct PlayerTip {
    pub player_id: i32,
    pub tip_id: i32,
}
