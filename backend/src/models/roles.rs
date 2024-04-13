use crate::{schema, models::{types::*, users::*}};
use diesel::prelude::*;

#[derive(Clone)]
#[derive(Queryable, Selectable, Identifiable, Insertable, AsChangeset)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(id))]
#[diesel(table_name = schema::roles)]
pub struct Role {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub name: RoleType
}

#[derive(Clone)]
#[derive(Queryable, Selectable, Identifiable, Insertable, Associations, AsChangeset)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(id))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Role))]
#[diesel(table_name = schema::user_roles)]
pub struct UserRole {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub user_id: i32,
    pub role_id: i32
}

#[derive(Clone)]
#[derive(Queryable, Selectable, Identifiable, Insertable, Associations, AsChangeset)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Admin))]
#[diesel(belongs_to(Role))]
#[diesel(table_name = schema::admin_roles)]
pub struct AdminRole {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub admin_id: i32,
    pub role_id: i32
}
