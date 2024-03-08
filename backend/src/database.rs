use crate::{config::CONFIG, models, schema};
use actix_web::web;
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
    ) -> anyhow::Result<PooledConnection<ConnectionManager<MysqlConnection>>> {
        self.pool.get().map_err(|e| anyhow::anyhow!(e))
    }

    pub async fn query_wrapper<F, T>(&self, f: F) -> anyhow::Result<T>
    where
        F: FnOnce(&mut MysqlConnection) -> Result<T, diesel::result::Error> + Send + 'static,
        T: Send + 'static,
    {
        let mut conn = self.get_connection()?;
        let result = web::block(move || f(&mut conn))
            .await
            .map_err(|e| {
                log::error!("Database error: {:?}", e);
                anyhow::anyhow!(e)
            })?.map_err(|e| {
                log::error!("Database error: {:?}", e);
                anyhow::anyhow!(e)
            })?;
        Ok(result)
    }

    pub async fn get_admin_by_username(
        &self,
        username: String,
    ) -> anyhow::Result<Option<models::Admin>> {
        self.query_wrapper(move |conn| {
            schema::admins::table
                .filter(schema::admins::username.eq(username))
                .first::<models::Admin>(conn)
                .optional()
        })
        .await
    }

    pub async fn create_admin(&self, new_admin: models::Admin) -> anyhow::Result<i32> {
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
    ) -> anyhow::Result<Option<models::User>> {
        self.query_wrapper(move |conn| {
            schema::users::table
                .filter(schema::users::username.eq(username))
                .first::<models::User>(conn)
                .optional()
        })
        .await
    }

    pub async fn get_user_by_id(&self, id: i32) -> anyhow::Result<Option<models::User>> {
        self.query_wrapper(move |conn| {
            schema::users::table
                .find(id)
                .first::<models::User>(conn)
                .optional()
        })
        .await
    }

    pub async fn update_user(&self, user: models::User) -> anyhow::Result<()> {
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

    pub async fn create_user(&self, new_user: models::User) -> anyhow::Result<i32> {
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

    pub async fn get_users(&self) -> anyhow::Result<Vec<models::User>> {
        self.query_wrapper(move |conn| schema::users::table.load::<models::User>(conn))
            .await
    }

    pub async fn get_statistics(
        &self,
        user_id: i32,
    ) -> anyhow::Result<Vec<models::StatisticsSample>> {
        self.query_wrapper(move |conn| {
            schema::statistics::table
                .filter(schema::statistics::user_id.eq(user_id))
                .load::<models::StatisticsSample>(conn)
        })
        .await
    }

    pub async fn create_statistics(
        &self,
        new_statistics: models::StatisticsSample,
    ) -> anyhow::Result<()> {
        self.query_wrapper(move |conn| {
            diesel::insert_into(schema::statistics::table)
                .values(&new_statistics)
                .execute(conn)
        })
        .await?;

        Ok(())
    }

    pub async fn unsert_crop_types(
        &self,
        crop_type: models::CropType,
    ) -> anyhow::Result<()> {
        self.query_wrapper(move |conn| {
            diesel::insert_into(schema::crop_types::table)
                .values(&crop_type)
                .on_conflict(diesel::dsl::DuplicatedKeys)
                .do_update()
                .set(&crop_type)
                .execute(conn)
        })
        .await?;

        Ok(())
    }

    pub async fn upsert_crop_sections(
        &self,
        new_crop_sections: Vec<models::CropSection>,
    ) -> anyhow::Result<()> {
        self.query_wrapper(move |conn| {
            conn.transaction(|pooled| {
                for crop_section in new_crop_sections {
                    diesel::insert_into(schema::crop_sections::table)
                        .values(&crop_section)
                        .on_conflict(diesel::dsl::DuplicatedKeys)
                        .do_update()
                        .set(&crop_section)
                        .execute(pooled)?;
                }

                Ok(())
            })
        })
        .await?;

        Ok(())
    }

    pub async fn get_crop_sections_by_user_id(
        &self,
        user_id: i32,
    ) -> anyhow::Result<Vec<models::CropSection>> {
        self.query_wrapper(move |conn| {
            schema::crop_sections::table
                .filter(schema::crop_sections::user_id.eq(user_id))
                .load::<models::CropSection>(conn)
        })
        .await
    }
}
