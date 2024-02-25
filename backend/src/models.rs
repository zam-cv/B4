use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::{admins, users};

#[derive(Clone, Serialize, Deserialize, Debug, Queryable, Selectable)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(id))]
#[diesel(table_name = admins)]
pub struct Admin {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable, Deserialize, Serialize, Debug)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(table_name = admins)]
pub struct NewAdmin {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Queryable, Selectable)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(id))]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable, Deserialize, Serialize, Debug)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}