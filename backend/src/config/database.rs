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
        if let Ok(None) = database.get_role_by_name(role).await {
            let role = models::Role {
                id: None,
                name: role,
            };

            database.create_role(role).await?;
        }
    }

    Ok(())
}

pub async fn create_default_permissions(database: &Database) -> anyhow::Result<()> {
    for permission in models::PermissionType::iter() {
        if let Ok(None) = database.get_permission_by_name(permission).await {
            let permission = models::Permission {
                id: None,
                name: permission,
            };

            database.create_permission(permission).await?;
        }
    }

    Ok(())
}
