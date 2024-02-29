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

    pub async fn create_admin(&self, new_admin: models::Admin) -> Result<i32, error::Error> {
        let mut conn = self.get_connection()?;
        let id = web::block(move || {
            conn.transaction(|pooled| {
                diesel::insert_into(schema::admins::table)
                    .values(&new_admin)
                    .execute(pooled)?;

                schema::admins::table
                    .select(schema::admins::id)
                    .order(schema::admins::id.desc())
                    .first::<i32>(pooled)
            })
        })
        .await?
        .map_err(error::ErrorInternalServerError)?;

        Ok(id)
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

    pub async fn create_user(&self, new_user: models::User) -> Result<i32, error::Error> {
        let mut conn = self.get_connection()?;
        let id = web::block(move || {
            conn.transaction(|pooled| {
                diesel::insert_into(schema::users::table)
                    .values(&new_user)
                    .execute(pooled)?;

                schema::users::table
                    .select(schema::users::id)
                    .order(schema::users::id.desc())
                    .first::<i32>(pooled)
            })
        })
        .await?
        .map_err(error::ErrorInternalServerError)?;

        Ok(id)
    }

    pub async fn get_users(&self) -> Result<Vec<models::User>, error::Error> {
        let mut conn = self.get_connection()?;
        let users = web::block(move || schema::users::table.load::<models::User>(&mut conn))
            .await?
            .map_err(error::ErrorInternalServerError)?;

        Ok(users)
    }

    pub async fn get_statistics(
        &self,
        user_id: i32,
    ) -> Result<Vec<models::StatisticsSample>, error::Error> {
        let mut conn = self.get_connection()?;
        let statistics = web::block(move || {
            schema::statistics::table
                .filter(schema::statistics::user_id.eq(user_id))
                .load::<models::StatisticsSample>(&mut conn)
        })
        .await?
        .map_err(error::ErrorInternalServerError)?;

        Ok(statistics)
    }

    #[allow(dead_code)]
    pub async fn create_statistics(
        &self,
        new_statistics: models::StatisticsSample,
    ) -> Result<(), error::Error> {
        let mut conn = self.get_connection()?;
        web::block(move || {
            diesel::insert_into(schema::statistics::table)
                .values(&new_statistics)
                .execute(&mut conn)
        })
        .await?
        .map_err(error::ErrorInternalServerError)?;

        Ok(())
    }
}
