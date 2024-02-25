use crate::{config::CONFIG, models, schema};
use actix_web::{error, web};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};

const MAX_POOL_SIZE: u32 = 5;
pub type DBPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[derive(Clone)]
pub struct Database {
    pub pool: DBPool,
}

impl Database {
    pub fn new() -> Self {
        let manager = ConnectionManager::<MysqlConnection>::new(CONFIG.database_url.clone());
        let pool = r2d2::Pool::builder()
            .max_size(MAX_POOL_SIZE)
            .build(manager)
            .expect("Failed to create pool.");

        Database { pool }
    }

    pub fn get_connection(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<MysqlConnection>>, error::Error> {
        self.pool.get().map_err(error::ErrorInternalServerError)
    }

    pub async fn get_admin_by_username(
        &self,
        username: String,
    ) -> Result<Option<models::Admin>, error::Error> {
        let mut conn = self.get_connection()?;
        let admin = web::block(move || {
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
        let mut conn = self.get_connection()?;
        let admin = web::block(move || {
            diesel::insert_into(schema::admins::table)
                .values(&new_admin)
                .execute(&mut conn)
        })
        .await?
        .map_err(error::ErrorInternalServerError)?;

        Ok(admin)
    }

    pub async fn get_user_by_username(
        &self,
        username: String,
    ) -> Result<Option<models::User>, error::Error> {
        let mut conn = self.get_connection()?;
        let user = web::block(move || {
            schema::users::table
                .filter(schema::users::username.eq(username))
                .first::<models::User>(&mut conn)
                .optional()
        })
        .await?
        .map_err(error::ErrorInternalServerError)?;

        Ok(user)
    }

    pub async fn create_user(&self, new_user: models::NewUser) -> Result<usize, error::Error> {
        let mut conn = self.get_connection()?;
        let user = web::block(move || {
            diesel::insert_into(schema::users::table)
                .values(&new_user)
                .execute(&mut conn)
        })
        .await?
        .map_err(error::ErrorInternalServerError)?;

        Ok(user)
    }
}
