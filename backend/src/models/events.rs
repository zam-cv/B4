use std::hash::Hash;

use crate::{models::{types::*, Statistic}, schema};
use diesel::prelude::*;

#[derive(Eq, PartialEq, Debug)]
#[derive(Queryable, Selectable, Identifiable, Insertable, AsChangeset)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(id))]
#[diesel(table_name = schema::events)]
pub struct Event {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub event_type: EventType,
    pub content: String,
}

impl Hash for Event {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Debug)]
#[derive(Queryable, Selectable, Associations, Identifiable, Insertable, AsChangeset)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Event))]
#[diesel(table_name = schema::functions)]
pub struct Function {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub function_type: FunctionType,
    pub event_id: i32,
    pub key: Option<String>,
    pub function: String,
}

#[derive(Debug)]
#[derive(Queryable, Selectable, Associations, Identifiable, Insertable, AsChangeset)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(primary_key(statistic_id, function_id))]
#[diesel(belongs_to(Statistic))]
#[diesel(belongs_to(Function))]
#[diesel(table_name = schema::values)]
pub struct Value {
    pub statistic_id: i32,
    pub function_id: i32,
    pub content: String
}
