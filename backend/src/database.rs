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

    pub async fn query_wrapper<F, T>(&self, f: F) -> Result<T, error::Error>
    where
        F: FnOnce(&mut MysqlConnection) -> Result<T, diesel::result::Error> + Send + 'static,
        T: Send + 'static,
    {
        let mut conn = self.get_connection()?;
        let result = web::block(move || f(&mut conn))
            .await?
            .map_err(error::ErrorInternalServerError)?;
        Ok(result)
    }

    pub async fn get_admin_by_username(
        &self,
        username: String,
    ) -> Result<Option<models::Admin>, error::Error> {
        self.query_wrapper(move |conn| {
            schema::admins::table
                .filter(schema::admins::username.eq(username))
                .first::<models::Admin>(conn)
                .optional()
        })
        .await
    }

    pub async fn create_admin(&self, new_admin: models::Admin) -> Result<i32, error::Error> {
        self.query_wrapper(move |conn| {
            conn.transaction(|pooled| {
                diesel::insert_into(schema::admins::table)
                    .values(&new_admin)
                    .execute(pooled)?;

                // Get the last inserted id
                schema::admins::table
                    .select(schema::admins::id)
                    .order(schema::admins::id.desc())
                    .first::<i32>(pooled)
            })
        })
        .await
    }

    pub async fn get_user_by_username(
        &self,
        username: String,
    ) -> Result<Option<models::User>, error::Error> {
        self.query_wrapper(move |conn| {
            schema::users::table
                .filter(schema::users::username.eq(username))
                .first::<models::User>(conn)
                .optional()
        })
        .await
    }

    pub async fn get_user_by_id(&self, id: i32) -> Result<Option<models::User>, error::Error> {
        self.query_wrapper(move |conn| {
            schema::users::table
                .find(id)
                .first::<models::User>(conn)
                .optional()
        })
        .await
    }

    pub async fn update_user(&self, user: models::User) -> Result<(), error::Error> {
        self.query_wrapper(move |conn| {
            if let Some(id) = &user.id {
                diesel::update(schema::users::table.find(id))
                    .set(&user)
                    .execute(conn)
            } else {
                Ok(0)
            }
        })
        .await?;

        Ok(())
    }

    pub async fn create_user(&self, new_user: models::User) -> Result<i32, error::Error> {
        self.query_wrapper(move |conn| {
            conn.transaction(|pooled| {
                diesel::insert_into(schema::users::table)
                    .values(&new_user)
                    .execute(pooled)?;

                // Get the last inserted id
                schema::users::table
                    .select(schema::users::id)
                    .order(schema::users::id.desc())
                    .first::<i32>(pooled)
            })
        })
        .await
    }

    pub async fn get_users(&self) -> Result<Vec<models::User>, error::Error> {
        self.query_wrapper(move |conn| schema::users::table.load::<models::User>(conn))
            .await
    }

    pub async fn get_statistics(
        &self,
        user_id: i32,
    ) -> Result<Vec<models::StatisticsSample>, error::Error> {
        self.query_wrapper(move |conn| {
            schema::statistics::table
                .filter(schema::statistics::user_id.eq(user_id))
                .load::<models::StatisticsSample>(conn)
        })
        .await
    }

    #[allow(dead_code)]
    pub async fn create_statistics(
        &self,
        new_statistics: models::StatisticsSample,
    ) -> Result<(), error::Error> {
        self.query_wrapper(move |conn| {
            diesel::insert_into(schema::statistics::table)
                .values(&new_statistics)
                .execute(conn)
        })
        .await?;

        Ok(())
    }
}
