use crate::{schema, models::*};

#[derive(Clone)]
#[derive(Queryable, Selectable, Identifiable, Insertable)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(name))]
#[diesel(table_name = schema::roles)]
pub struct Role {
    pub name: String
}

#[derive(Clone)]
#[derive(Queryable, Selectable, Identifiable, Insertable, Associations)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(admin_id, permission_id))]
#[diesel(belongs_to(Admin))]
#[diesel(belongs_to(Permission))]
#[diesel(table_name = schema::admin_permissions)]
pub struct AdminPermissions {
    pub admin_id: i32,
    pub permission_id: String
}
