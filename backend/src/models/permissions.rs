use crate::{schema, models::*};

#[derive(Clone)]
#[derive(Queryable, Selectable, Identifiable, Insertable)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(name))]
#[diesel(table_name = schema::permissions)]
pub struct Permission {
    pub name: String
}

#[derive(Clone)]
#[derive(Queryable, Selectable, Identifiable, Insertable, Associations, AsChangeset)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Role))]
#[diesel(belongs_to(Permission))]
#[diesel(table_name = schema::role_permissions)]
pub struct RolePermission {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub role_id: String,
    pub permission_id: String
}
