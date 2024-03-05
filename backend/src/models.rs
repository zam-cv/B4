use crate::schema;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[derive(Queryable, Selectable, Identifiable, Insertable)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(id))]
#[diesel(table_name = schema::admins)]
pub struct Admin {
    #[serde(skip_deserializing)]
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub username: String,
    pub password: String,
}

#[derive(Clone, Deserialize, Serialize)]
#[derive(Queryable, Selectable, Identifiable, Insertable, AsChangeset)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(id))]
#[diesel(table_name = schema::users)]
pub struct User {
    #[serde(skip_deserializing)]
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    #[serde(skip_deserializing)]
    pub balance_cash: i32,
    #[serde(skip_deserializing)]
    pub balance_verqor: i32,
    #[serde(skip_deserializing)]
    pub balance_coyote: i32,
    #[serde(skip_deserializing)]
    pub current_day: chrono::NaiveDateTime,
    #[serde(skip_deserializing)]
    pub max_sections: i32,
}

#[derive(Serialize)]
#[derive(Queryable, Selectable, Identifiable, Insertable, Associations)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(id))]
#[diesel(belongs_to(User))]
#[diesel(table_name = schema::statistics)]
pub struct StatisticsSample {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    #[serde(skip_serializing)]
    pub user_id: i32,
    #[serde(skip_serializing)]
    pub date: chrono::NaiveDateTime,
    pub punctuation: i32,
}

#[derive(Deserialize)]
#[derive(Queryable, Selectable, Identifiable, Insertable)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(id))]
#[diesel(table_name = schema::crop_types)]
pub struct CropType {
    #[serde(skip_deserializing)]
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub name: String,
    pub price: i32,
}

#[derive(Clone, Serialize)]
#[derive(Queryable, Selectable, Identifiable, Insertable, Associations, AsChangeset)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(id))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(CropType))]
#[diesel(table_name = schema::crop_sections)]
pub struct CropSection {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    #[serde(skip_deserializing)]
    pub user_id: i32,
    pub crop_type_id: Option<i32>,
    pub units: i32,
}