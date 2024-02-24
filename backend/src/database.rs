use crate::{config::CONFIG, models, schema};
use actix_web::{error, web};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub type DBPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[derive(Clone)]
pub struct Database {
    pub pool: DBPool,
}

impl Database {
    pub fn new() -> Self {
        let manager = ConnectionManager::<MysqlConnection>::new(CONFIG.database_url.clone());
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        Database { pool }
    }

    pub async fn get_admin_by_username(
        &self,
        username: String,
    ) -> Result<Option<models::Admin>, error::Error> {
        let pool = self.pool.clone();
        let admin = web::block(move || {
            let mut conn = pool.get().unwrap();
            schema::admins::table
                .filter(schema::admins::username.eq(username))
                .first::<models::Admin>(&mut conn)
                .optional()
        })
        .await?
        .map_err(error::ErrorInternalServerError)?;

        Ok(admin)
    }

    pub async fn create_admin(&self, new_admin: models::NewAdmin) -> Result<usize, error::Error> {
        let pool = self.pool.clone();
        let admin = web::block(move || {
            let mut conn = pool.get().unwrap();
            diesel::insert_into(schema::admins::table)
                .values(&new_admin)
                .execute(&mut conn)
        })
        .await?
        .map_err(error::ErrorInternalServerError)?;

        Ok(admin)
    }
}
