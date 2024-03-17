use crate::schema;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;
use diesel_derive_enum::DbEnum;
use rand_derive::Rand;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(DbEnum, Serialize, Deserialize, Rand)]
pub enum Gender {
    M,
    F
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(DbEnum, Serialize, Deserialize, Rand)]
pub enum UserType {
    Inversionista,
    Agricultor
}

#[derive(Serialize, Deserialize, Validate)]
#[derive(Queryable, Selectable, Identifiable, Insertable)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(id))]
#[diesel(table_name = schema::admins)]
pub struct Admin {
    #[serde(skip_deserializing, skip_serializing)]
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    #[validate(email)]
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Clone, Deserialize, Serialize, Validate, Debug, PartialEq)]
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
    #[serde(skip_serializing)]
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
}

#[derive(Clone, Deserialize, Serialize)]
#[derive(Queryable, Selectable, Identifiable, Insertable, AsChangeset)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(id))]
#[diesel(belongs_to(User))]
#[diesel(table_name = schema::players)]
pub struct Player {
    #[serde(skip_deserializing, skip_serializing)]
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub current_cycle: i32,
    pub current_score: i32,
    pub current_balance: i32,
    pub max_plots: i32,
}

#[derive(Clone, Deserialize)]
#[derive(Queryable, Selectable, Identifiable, Insertable, AsChangeset)]
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

#[derive(Clone, Deserialize)]
#[derive(Queryable, Selectable, Identifiable, Insertable, AsChangeset)]
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

#[derive(Serialize)]
#[derive(Queryable, Selectable, Identifiable, Insertable, Associations)]
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
    pub player_id: i32
}

#[derive(Deserialize, Validate)]
#[derive(Queryable, Selectable, Identifiable, Insertable, AsChangeset)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(name))]
#[diesel(table_name = schema::crop_types)]
pub struct CropType {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(range(min = 1))]
    pub price: i32,
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
    pub player_id: i32
}

pub(crate) mod exports {
    pub use super::GenderMapping as Gender;
    pub use super::UserTypeMapping as UserType;
}