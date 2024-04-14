use crate::{
    config::{self, CONFIG},
    models, schema,
};
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
            })?
            .map_err(|e| {
                log::error!("Database error: {:?}", e);
                anyhow::anyhow!(e)
            })?;
        Ok(result)
    }

    pub async fn get_admin_by_id(&self, id: i32) -> anyhow::Result<Option<models::Admin>> {
        self.query_wrapper(move |conn| {
            schema::admins::table
                .find(id)
                .first::<models::Admin>(conn)
                .optional()
        })
        .await
    }

    pub async fn get_admin_by_email(&self, email: String) -> anyhow::Result<Option<models::Admin>> {
        self.query_wrapper(move |conn| {
            schema::admins::table
                .filter(schema::admins::email.eq(email))
                .first::<models::Admin>(conn)
                .optional()
        })
        .await
    }

    pub async fn get_admins(&self) -> anyhow::Result<Vec<models::Admin>> {
        self.query_wrapper(move |conn| {
            schema::admins::table
                .filter(schema::admins::email.ne(&CONFIG.admin_default_email))
                .load::<models::Admin>(conn)
        })
        .await
    }

    pub async fn delete_admin_by_id(&self, id: i32) -> anyhow::Result<()> {
        self.query_wrapper(move |conn| {
            diesel::delete(schema::admins::table.find(id)).execute(conn)
        })
        .await?;

        Ok(())
    }

    pub async fn create_admin(
        &self,
        new_admin: models::Admin,
        permissions: Vec<models::PermissionType>,
    ) -> anyhow::Result<i32> {
        self.query_wrapper(move |conn| {
            conn.transaction(|pooled| {
                diesel::insert_into(schema::admins::table)
                    .values(&new_admin)
                    .execute(pooled)?;

                // Get the last inserted id
                let id = schema::admins::table
                    .select(schema::admins::id)
                    .order(schema::admins::id.desc())
                    .first::<i32>(pooled)?;

                let permissions = permissions
                    .into_iter()
                    .map(|permission| models::AdminPermissions {
                        id: None,
                        admin_id: id,
                        permission_id: permission.to_string(),
                    })
                    .collect::<Vec<_>>();

                diesel::insert_into(schema::admin_permissions::table)
                    .values(&permissions)
                    .execute(pooled)?;

                Ok(id)
            })
        })
        .await
    }

    pub async fn get_user_by_email(&self, email: String) -> anyhow::Result<Option<models::User>> {
        self.query_wrapper(move |conn| {
            schema::users::table
                .filter(schema::users::email.eq(email))
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

    pub async fn get_players_count(&self) -> anyhow::Result<i64> {
        self.query_wrapper(move |conn| schema::players::table.count().get_result::<i64>(conn))
            .await
    }

    #[allow(dead_code)]
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

    pub async fn get_player_by_id(&self, id: i32) -> anyhow::Result<Option<models::Player>> {
        self.query_wrapper(move |conn| {
            schema::players::table
                .find(id)
                .first::<models::Player>(conn)
                .optional()
        })
        .await
    }

    pub async fn get_player_by_user_id(
        &self,
        user_id: i32,
    ) -> anyhow::Result<Option<models::Player>> {
        self.query_wrapper(move |conn| {
            schema::users::table
                .find(user_id)
                .inner_join(schema::players::table)
                .select(schema::players::all_columns)
                .first::<models::Player>(conn)
                .optional()
        })
        .await
    }

    pub async fn update_player(&self, player: models::Player) -> anyhow::Result<()> {
        self.query_wrapper(move |conn| {
            if let Some(id) = &player.id {
                diesel::update(schema::players::table.find(id))
                    .set(&player)
                    .execute(conn)
            } else {
                Ok(0)
            }
        })
        .await?;

        Ok(())
    }

    pub async fn create_player(&self) -> anyhow::Result<i32> {
        self.query_wrapper(move |conn| {
            let new_player = models::Player {
                id: None,
                current_cycle: config::INITIAL_CYCLE,
                current_score: config::INITIAL_SCORE,
                balance_cash: config::INITIAL_BALANCE_CASH,
                balance_verqor: config::INITIAL_BALANCE,
                balance_coyote: config::INITIAL_BALANCE,
                max_plots: config::INITIAL_MAX_PLOTS,
            };

            conn.transaction(|pooled| {
                diesel::insert_into(schema::players::table)
                    .values(&new_player)
                    .execute(pooled)?;

                // Get the last inserted id
                schema::players::table
                    .select(schema::players::id)
                    .order(schema::players::id.desc())
                    .first::<i32>(pooled)
            })
        })
        .await
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
        player_id: i32,
    ) -> anyhow::Result<Vec<models::StatisticsSample>> {
        self.query_wrapper(move |conn| {
            schema::statistics::table
                .filter(schema::statistics::player_id.eq(player_id))
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

    pub async fn unsert_crop_types(&self, crop_type: models::CropType) -> anyhow::Result<()> {
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

    pub async fn upsert_plots(&self, new_plots: Vec<models::Plot>) -> anyhow::Result<()> {
        self.query_wrapper(move |conn| {
            conn.transaction(|pooled| {
                for plot in new_plots {
                    diesel::insert_into(schema::plots::table)
                        .values(&plot)
                        .on_conflict(diesel::dsl::DuplicatedKeys)
                        .do_update()
                        .set(&plot)
                        .execute(pooled)?;
                }

                Ok(())
            })
        })
        .await?;

        Ok(())
    }

    pub async fn get_plots_by_player_id(
        &self,
        player_id: i32,
    ) -> anyhow::Result<Vec<models::Plot>> {
        self.query_wrapper(move |conn| {
            schema::plots::table
                .filter(schema::plots::player_id.eq(player_id))
                .load::<models::Plot>(conn)
        })
        .await
    }

    pub async fn create_role(&self, new_role: models::Role) -> anyhow::Result<()> {
        self.query_wrapper(move |conn| {
            diesel::insert_into(schema::roles::table)
                .values(&new_role)
                .execute(conn)
        })
        .await?;

        Ok(())
    }

    pub async fn get_role(
        &self,
        role_type: models::RoleType,
    ) -> anyhow::Result<Option<models::Role>> {
        self.query_wrapper(move |conn| {
            schema::roles::table
                .filter(schema::roles::name.eq(role_type.to_string()))
                .first::<models::Role>(conn)
                .optional()
        })
        .await
    }

    pub async fn get_permission(
        &self,
        permission_type: models::PermissionType,
    ) -> anyhow::Result<Option<models::Permission>> {
        self.query_wrapper(move |conn| {
            schema::permissions::table
                .filter(schema::permissions::name.eq(permission_type.to_string()))
                .first::<models::Permission>(conn)
                .optional()
        })
        .await
    }

    pub async fn create_permission(
        &self,
        new_permission: models::Permission,
    ) -> anyhow::Result<()> {
        self.query_wrapper(move |conn| {
            diesel::insert_into(schema::permissions::table)
                .values(&new_permission)
                .execute(conn)
        })
        .await?;

        Ok(())
    }

    pub async fn create_role_permission(
        &self,
        new_role_permission: models::RolePermission,
    ) -> anyhow::Result<()> {
        self.query_wrapper(move |conn| {
            diesel::insert_into(schema::role_permissions::table)
                .values(&new_role_permission)
                .execute(conn)
        })
        .await?;

        Ok(())
    }

    pub async fn get_role_permission(
        &self,
        role_name: String,
        permission_name: String,
    ) -> anyhow::Result<Option<models::RolePermission>> {
        self.query_wrapper(move |conn| {
            schema::role_permissions::table
                .filter(
                    schema::role_permissions::role_id
                        .eq(role_name)
                        .and(schema::role_permissions::permission_id.eq(permission_name)),
                )
                .first::<models::RolePermission>(conn)
                .optional()
        })
        .await
    }

    pub async fn get_permissions_by_admin_id(
        &self,
        admin_id: i32,
    ) -> anyhow::Result<Vec<models::PermissionType>> {
        self.query_wrapper(move |conn| {
            schema::admin_permissions::table
                .filter(schema::admin_permissions::admin_id.eq(admin_id))
                .select(schema::admin_permissions::permission_id)
                .load::<String>(conn)
        })
        .await
        .map(|permissions| {
            permissions
                .into_iter()
                .map(|permission| permission.parse().unwrap())
                .collect()
        })
    }
}
