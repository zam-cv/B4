use crate::schema::{admins, statistics, users};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Queryable, Selectable, Identifiable, Insertable)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(id))]
#[diesel(table_name = admins)]
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
#[diesel(table_name = users)]
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
}

#[derive(Serialize, Queryable, Selectable, Identifiable, Insertable, Associations)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(id))]
#[diesel(belongs_to(User))]
#[diesel(table_name = statistics)]
pub struct StatisticsSample {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    #[serde(skip_serializing)]
    pub user_id: i32,
    #[serde(skip_serializing)]
    pub date: chrono::NaiveDateTime,
    pub punctuation: i32,
}
