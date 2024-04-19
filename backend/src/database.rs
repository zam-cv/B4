use crate::{config::CONFIG, models, routes::admin::mail::Filters, schema};
use actix_web::web;
use chrono::Datelike;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};
use std::collections::HashMap;

macro_rules! count_star {
    ($type:ty) => {
        diesel::dsl::sql::<$type>("count(*)")
    };
}

macro_rules! avg {
    ($column:expr) => {
        diesel::dsl::sql::<diesel::sql_types::Nullable<diesel::sql_types::Double>>(concat!(
            "AVG(", $column, ")"
        ))
    };
}

macro_rules! apply_filters {
    ($box:expr, $filters:expr) => {
        if let Some((min, max)) = $filters.by_age_range {
            // get the minimum and maximum years by ages
            let current_year = chrono::Utc::now().year();
            let min_year = current_year - max;
            let max_year = current_year - min;

            $box = $box.filter(
                schema::users::year_of_birth
                    .ge(min_year)
                    .and(schema::users::year_of_birth.lt(max_year)),
            );
        }

        if let Some(user_type) = $filters.by_user_type {
            $box = $box.filter(schema::users::user_type.eq(user_type));
        }

        if let Some(gender) = $filters.by_gender {
            $box = $box.filter(schema::users::gender.eq(gender));
        }

        if let Some(extension) = $filters.by_extension {
            $box = $box.filter(schema::users::email.like(format!("%{}%", extension)));
        }
    };
}

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
        self.delete_permissions_by_admin_id(id).await?;

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
        let id = self
            .query_wrapper(move |conn| {
                conn.transaction(|pooled| {
                    diesel::insert_into(schema::admins::table)
                        .values(&new_admin)
                        .execute(pooled)?;

                    // Get the last inserted id
                    let id = schema::admins::table
                        .select(schema::admins::id)
                        .order(schema::admins::id.desc())
                        .first::<i32>(pooled)?;

                    Ok(id)
                })
            })
            .await?;

        self.set_permissions_by_admin_id(id, permissions).await?;
        Ok(id)
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

    pub async fn create_player(&self, new_player: models::Player) -> anyhow::Result<i32> {
        self.query_wrapper(move |conn| {
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

    pub async fn get_average_time_in_game(&self) -> anyhow::Result<Option<f64>> {
        self.query_wrapper(move |conn| {
            schema::players::table
                .select(diesel::dsl::avg(schema::players::time_in_game))
                .first::<Option<f64>>(conn)
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

    pub async fn get_user_count_by_type(&self) -> anyhow::Result<Vec<(models::UserType, i64)>> {
        self.query_wrapper(move |conn| {
            schema::users::table
                .select((
                    schema::users::user_type,
                    count_star!(diesel::sql_types::BigInt),
                ))
                .group_by(schema::users::user_type)
                .load::<(models::UserType, i64)>(conn)
        })
        .await
    }

    pub async fn get_max_year_of_birth(&self) -> anyhow::Result<Option<i32>> {
        self.query_wrapper(move |conn| {
            schema::users::table
                .select(diesel::dsl::max(schema::users::year_of_birth))
                .first::<Option<i32>>(conn)
        })
        .await
    }

    pub async fn get_min_year_of_birth(&self) -> anyhow::Result<Option<i32>> {
        self.query_wrapper(move |conn| {
            schema::users::table
                .select(diesel::dsl::min(schema::users::year_of_birth))
                .first::<Option<i32>>(conn)
        })
        .await
    }

    pub async fn get_average_age(&self) -> anyhow::Result<Option<f64>> {
        let current_year = chrono::Utc::now().year();

        let average = self
            .query_wrapper(move |conn| {
                schema::users::table
                    .select(avg!("year_of_birth"))
                    .first::<Option<f64>>(conn)
            })
            .await?;

        Ok(average.map(|average| current_year as f64 - average))
    }

    pub async fn get_user_count_by_age_range(
        &self,
        step: i32,
    ) -> anyhow::Result<Vec<(String, i64)>> {
        let current_year = chrono::Utc::now().year();
        let max = self.get_max_year_of_birth().await?;
        let min = self.get_min_year_of_birth().await?;

        if let (Some(max), Some(min)) = (max, min) {
            let mut ranges = Vec::new();
            let mut start = min;
            let mut end = start + step;

            while start < max {
                let range = format!("{}-{}", current_year - start, current_year - end);
                let count = self
                    .query_wrapper(move |conn| {
                        schema::users::table
                            .filter(
                                schema::users::year_of_birth.ge(start).and(
                                    schema::users::year_of_birth
                                        .lt(end)
                                        .or(schema::users::year_of_birth.eq(end)),
                                ),
                            )
                            .select(count_star!(diesel::sql_types::BigInt))
                            .first::<i64>(conn)
                    })
                    .await?;

                ranges.push((range, count));

                start = end;
                end = start + step;
            }

            Ok(ranges)
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn get_user_count_by_gender(&self) -> anyhow::Result<Vec<(models::Gender, i64)>> {
        self.query_wrapper(move |conn| {
            schema::users::table
                .select((
                    schema::users::gender,
                    count_star!(diesel::sql_types::BigInt),
                ))
                .group_by(schema::users::gender)
                .load::<(models::Gender, i64)>(conn)
        })
        .await
    }

    pub async fn get_user_locations_by_type(
        &self,
    ) -> anyhow::Result<Vec<(models::UserType, Vec<(Option<f64>, Option<f64>)>)>> {
        self.query_wrapper(move |conn| {
            schema::users::table
                .select((
                    schema::users::user_type,
                    (schema::users::latitude, schema::users::longitude),
                ))
                .filter(
                    schema::users::latitude
                        .is_not_null()
                        .and(schema::users::longitude.is_not_null()),
                )
                .load::<(models::UserType, (Option<f64>, Option<f64>))>(conn)
        })
        .await
        .map(|locations| {
            let mut map = std::collections::HashMap::new();
            for (user_type, location) in locations {
                map.entry(user_type).or_insert_with(Vec::new).push(location);
            }

            map.into_iter().collect()
        })
    }

    pub async fn get_average_time_in_game_by_user_type(
        &self,
    ) -> anyhow::Result<Vec<(models::UserType, Option<f64>)>> {
        self.query_wrapper(move |conn| {
            schema::players::table
                .inner_join(schema::users::table)
                .select((schema::users::user_type, avg!("time_in_game")))
                .group_by(schema::users::user_type)
                .load::<(models::UserType, Option<f64>)>(conn)
        })
        .await
    }

    pub async fn get_statistics(&self, player_id: i32) -> anyhow::Result<Vec<models::Statistic>> {
        self.query_wrapper(move |conn| {
            schema::statistics::table
                .filter(schema::statistics::player_id.eq(player_id))
                .load::<models::Statistic>(conn)
        })
        .await
    }

    pub async fn create_statistics(&self, new_statistics: models::Statistic) -> anyhow::Result<()> {
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

    pub async fn get_crop_types(&self) -> anyhow::Result<Vec<models::CropType>> {
        self.query_wrapper(move |conn| schema::crop_types::table.load::<models::CropType>(conn))
            .await
    }

    pub async fn get_crop_type_by_name(
        &self,
        name: String,
    ) -> anyhow::Result<Option<models::CropType>> {
        self.query_wrapper(move |conn| {
            schema::crop_types::table
                .filter(schema::crop_types::name.eq(name))
                .first::<models::CropType>(conn)
                .optional()
        })
        .await
    }

    pub async fn update_crop_type_description(
        &self,
        name: String,
        description: String,
    ) -> anyhow::Result<()> {
        self.query_wrapper(move |conn| {
            diesel::update(schema::crop_types::table.filter(schema::crop_types::name.eq(name)))
                .set(schema::crop_types::description.eq(description))
                .execute(conn)
        })
        .await?;

        Ok(())
    }

    pub async fn update_crop_type_price(&self, name: String, price: i32) -> anyhow::Result<()> {
        self.query_wrapper(move |conn| {
            diesel::update(schema::crop_types::table.filter(schema::crop_types::name.eq(name)))
                .set(schema::crop_types::price.eq(price))
                .execute(conn)
        })
        .await?;

        Ok(())
    }

    pub async fn update_crop_type_duration(
        &self,
        name: String,
        duration: i32,
    ) -> anyhow::Result<()> {
        self.query_wrapper(move |conn| {
            diesel::update(schema::crop_types::table.filter(schema::crop_types::name.eq(name)))
                .set(schema::crop_types::duration.eq(duration))
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
                .filter_map(|permission| match permission.parse() {
                    Ok(permission) => Some(permission),
                    Err(_) => None,
                })
                .collect()
        })
    }

    pub async fn get_permissions_by_role_id(
        &self,
        role_id: String,
    ) -> anyhow::Result<Vec<models::PermissionType>> {
        self.query_wrapper(move |conn| {
            schema::role_permissions::table
                .filter(schema::role_permissions::role_id.eq(role_id))
                .select(schema::role_permissions::permission_id)
                .load::<String>(conn)
        })
        .await
        .map(|permissions| {
            permissions
                .into_iter()
                .filter_map(|permission| match permission.parse() {
                    Ok(permission) => Some(permission),
                    Err(_) => None,
                })
                .collect()
        })
    }

    pub async fn set_permissions_by_admin_id(
        &self,
        admin_id: i32,
        permissions: Vec<models::PermissionType>,
    ) -> anyhow::Result<()> {
        self.query_wrapper(move |conn| {
            conn.transaction(|pooled| {
                diesel::delete(
                    schema::admin_permissions::table
                        .filter(schema::admin_permissions::admin_id.eq(admin_id)),
                )
                .execute(pooled)?;

                for permission in permissions {
                    diesel::insert_into(schema::admin_permissions::table)
                        .values(models::AdminPermissions {
                            admin_id,
                            permission_id: permission.to_string(),
                        })
                        .execute(pooled)?;
                }

                Ok(())
            })
        })
        .await
    }

    pub async fn unsert_permission_by_admin_id(
        &self,
        admin_id: i32,
        permission: models::PermissionType,
    ) -> anyhow::Result<()> {
        self.query_wrapper(move |conn| {
            diesel::insert_into(schema::admin_permissions::table)
                .values(models::AdminPermissions {
                    admin_id,
                    permission_id: permission.to_string(),
                })
                .execute(conn)
        })
        .await?;

        Ok(())
    }

    pub async fn delete_permission_by_admin_id(
        &self,
        admin_id: i32,
        permission: models::PermissionType,
    ) -> anyhow::Result<()> {
        self.query_wrapper(move |conn| {
            diesel::delete(
                schema::admin_permissions::table.filter(
                    schema::admin_permissions::admin_id
                        .eq(admin_id)
                        .and(schema::admin_permissions::permission_id.eq(permission.to_string())),
                ),
            )
            .execute(conn)
        })
        .await?;

        Ok(())
    }

    pub async fn delete_permissions_by_admin_id(&self, admin_id: i32) -> anyhow::Result<()> {
        self.query_wrapper(move |conn| {
            diesel::delete(
                schema::admin_permissions::table
                    .filter(schema::admin_permissions::admin_id.eq(admin_id)),
            )
            .execute(conn)
        })
        .await?;

        Ok(())
    }

    pub async fn upsert_session(&self, new_session: models::Session) -> anyhow::Result<()> {
        self.query_wrapper(move |conn| {
            conn.transaction(|pooled| {
                let existing = schema::sessions::table
                    .filter(schema::sessions::user_id.eq(new_session.user_id))
                    .filter(schema::sessions::created_at.eq(new_session.created_at))
                    .first::<models::Session>(pooled)
                    .optional()?;

                match existing {
                    Some(mut existing) => {
                        existing.times += 1;
                        diesel::update(
                            schema::sessions::table.find((existing.created_at, existing.user_id)),
                        )
                        .set(&existing)
                        .execute(pooled)?;
                    }
                    None => {
                        diesel::insert_into(schema::sessions::table)
                            .values(&new_session)
                            .execute(pooled)?;
                    }
                };

                Ok(())
            })
        })
        .await?;

        Ok(())
    }

    pub async fn get_average_sessions_by_day_of_week(
        &self,
    ) -> anyhow::Result<Vec<(i32, Option<f64>)>> {
        self.query_wrapper(move |conn| {
            schema::sessions::table
                .select((
                    diesel::dsl::sql::<diesel::sql_types::Integer>("DAYOFWEEK(created_at)"),
                    avg!("times"),
                ))
                .group_by(diesel::dsl::sql::<diesel::sql_types::Integer>(
                    "DAYOFWEEK(created_at)",
                ))
                .load::<(i32, Option<f64>)>(conn)
        })
        .await
    }

    pub async fn create_tip(&self, new_tip: models::Tip) -> anyhow::Result<i32> {
        self.query_wrapper(move |conn| {
            conn.transaction(|pooled| {
                diesel::insert_into(schema::tips::table)
                    .values(&new_tip)
                    .execute(pooled)?;

                // Get the last inserted id
                schema::tips::table
                    .select(schema::tips::id)
                    .order(schema::tips::id.desc())
                    .first::<i32>(pooled)
            })
        })
        .await
    }

    pub async fn get_tips(&self) -> anyhow::Result<Vec<models::Tip>> {
        self.query_wrapper(move |conn| schema::tips::table.load::<models::Tip>(conn))
            .await
    }

    pub async fn update_tip(&self, tip: models::Tip) -> anyhow::Result<()> {
        self.query_wrapper(move |conn| {
            if let Some(id) = &tip.id {
                diesel::update(schema::tips::table.find(id))
                    .set(&tip)
                    .execute(conn)
            } else {
                Err(diesel::result::Error::NotFound)
            }
        })
        .await?;

        Ok(())
    }

    pub async fn delete_tip_by_id(&self, id: i32) -> anyhow::Result<()> {
        self.query_wrapper(move |conn| {
            diesel::delete(schema::player_tips::table.filter(schema::player_tips::tip_id.eq(id)))
                .execute(conn)
        })
        .await?;

        self.query_wrapper(move |conn| diesel::delete(schema::tips::table.find(id)).execute(conn))
            .await?;

        Ok(())
    }

    pub async fn get_tip_by_content(&self, content: String) -> anyhow::Result<Option<models::Tip>> {
        self.query_wrapper(move |conn| {
            schema::tips::table
                .filter(schema::tips::content.eq(content))
                .first::<models::Tip>(conn)
                .optional()
        })
        .await
    }

    pub async fn get_random_tip(&self, player_id: i32) -> anyhow::Result<Option<models::Tip>> {
        self.query_wrapper(move |conn| {
            let count: i64 = schema::tips::table
                .filter(
                    schema::tips::id.ne_all(
                        schema::player_tips::table
                            .select(schema::player_tips::tip_id)
                            .filter(schema::player_tips::player_id.eq(player_id)),
                    ),
                )
                .select(diesel::dsl::count_star())
                .first(conn)?;

            if count > 0 {
                let random_offset = rand::random::<usize>() % count as usize;

                let result = schema::tips::table
                    .filter(
                        schema::tips::id.ne_all(
                            schema::player_tips::table
                                .select(schema::player_tips::tip_id)
                                .filter(schema::player_tips::player_id.eq(player_id)),
                        ),
                    )
                    .offset(random_offset as i64)
                    .limit(1)
                    .first::<models::Tip>(conn)
                    .optional()?;

                return Ok(result);
            }

            Ok(None)
        })
        .await
    }

    pub async fn register_tip(&self, tip_id: i32, player_id: i32) -> anyhow::Result<()> {
        self.query_wrapper(move |conn| {
            diesel::insert_into(schema::player_tips::table)
                .values(models::PlayerTip { player_id, tip_id })
                .execute(conn)
        })
        .await?;

        Ok(())
    }

    pub async fn get_user_count_by_user_filter(&self, filters: Filters) -> anyhow::Result<i64> {
        self.query_wrapper(move |conn| {
            let mut query = schema::users::table.into_boxed();
            apply_filters!(query, filters);
            query.count().get_result::<i64>(conn)
        })
        .await
    }

    pub async fn get_emails_by_user_filter(&self, filters: Filters) -> anyhow::Result<Vec<String>> {
        self.query_wrapper(move |conn| {
            let mut query = schema::users::table.into_boxed();
            apply_filters!(query, filters);
            query.select(schema::users::email).load::<String>(conn)
        })
        .await
    }

    pub async fn create_event(&self, new_event: models::Event) -> anyhow::Result<i32> {
        self.query_wrapper(move |conn| {
            conn.transaction(|pooled| {
                diesel::insert_into(schema::events::table)
                    .values(&new_event)
                    .execute(pooled)?;

                // Get the last inserted id
                schema::events::table
                    .select(schema::events::id)
                    .order(schema::events::id.desc())
                    .first::<i32>(pooled)
            })
        })
        .await
    }

    pub async fn exists_event_by_type_and_content(
        &self,
        event_type: models::EventType,
        content: String,
    ) -> anyhow::Result<bool> {
        self.query_wrapper(move |conn| {
            let count = schema::events::table
                .filter(schema::events::event_type.eq(event_type))
                .filter(schema::events::content.eq(content))
                .count()
                .get_result::<i64>(conn)?;

            Ok(count > 0)
        })
        .await
    }

    pub async fn create_function(&self, new_function: models::Function) -> anyhow::Result<i32> {
        self.query_wrapper(move |conn| {
            conn.transaction(|pooled| {
                diesel::insert_into(schema::functions::table)
                    .values(&new_function)
                    .execute(pooled)?;

                // Get the last inserted id
                schema::functions::table
                    .select(schema::functions::id)
                    .order(schema::functions::id.desc())
                    .first::<i32>(pooled)
            })
        })
        .await
    }

    pub async fn get_history_by_player_id(
        &self,
        player_id: i32,
    ) -> anyhow::Result<
        Vec<(
            models::Statistic,
            Vec<(models::Event, Vec<(models::Function, models::Value)>)>,
        )>,
    > {
        self.query_wrapper(move |conn| {
            let data: Vec<(
                models::Statistic,
                models::Event,
                models::Function,
                models::Value,
            )> = schema::values::table
                .inner_join(schema::functions::table.inner_join(schema::events::table))
                .inner_join(schema::statistics::table)
                .filter(schema::statistics::player_id.eq(player_id))
                .select((
                    schema::statistics::all_columns,
                    schema::events::all_columns,
                    schema::functions::all_columns,
                    schema::values::all_columns,
                ))
                .load::<(
                    models::Statistic,
                    models::Event,
                    models::Function,
                    models::Value,
                )>(conn)?;
            
            let mut statistics = HashMap::new();

            for (statistic, event, function, value) in data {
                statistics.entry(statistic)
                    .or_insert_with(Vec::new)
                    .push((event, function, value));
            }

            let mut result = Vec::new();

            for (statistic, data) in statistics {
                let mut events = HashMap::new();

                for (event, function, value) in data {
                    events.entry(event)
                        .or_insert_with(Vec::new)
                        .push((function, value));
                }

                result.push((statistic, events.into_iter().collect()));
            }

            Ok(result)
        })
        .await
    }
}
