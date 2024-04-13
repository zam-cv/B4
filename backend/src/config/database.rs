use crate::{config::CONFIG, database::Database, models, utils};
use strum::IntoEnumIterator;

pub async fn create_default_admin(database: &Database) -> anyhow::Result<i32> {
    match database
        .get_admin_by_email(CONFIG.admin_default_email.clone())
        .await
    {
        Ok(None) => {
            if let Ok(password) = utils::get_hash_in_string(&CONFIG.admin_default_password) {
                let admin = models::Admin {
                    id: None,
                    email: CONFIG.admin_default_email.clone(),
                    password,
                    role_id: models::RoleType::Admin.to_string(),
                };

                if let Ok(id) = database.create_admin(admin).await {
                    log::info!("Default admin created");
                    return Ok(id);
                }
            }
        }
        Ok(Some(admin)) => {
            log::info!("Default admin already exists");
            return Ok(admin.id.unwrap());
        }
        _ => {}
    };

    Err(anyhow::anyhow!("Failed to create default admin"))
}

pub async fn create_default_roles(database: &Database) -> anyhow::Result<()> {
    for role in models::RoleType::iter() {
        if let Ok(None) = database.get_role(role).await {
            database
                .create_role(models::Role {
                    name: role.to_string(),
                })
                .await?;
        }
    }

    Ok(())
}

pub async fn create_default_permissions(database: &Database) -> anyhow::Result<()> {
    for permission in models::PermissionType::iter() {
        if let Ok(None) = database.get_permission(permission).await {
            database
                .create_permission(models::Permission {
                    name: permission.to_string(),
                })
                .await?;
        }
    }

    Ok(())
}
