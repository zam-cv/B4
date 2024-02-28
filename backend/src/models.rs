use crate::schema::{admins, users};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Selectable, Identifiable, Insertable)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(id))]
#[diesel(table_name = admins)]
pub struct Admin {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Queryable, Selectable, Identifiable, Insertable)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(id))]
#[diesel(table_name = users)]
pub struct User {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub username: String,
    pub password: String,
}
