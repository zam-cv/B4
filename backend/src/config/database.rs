use crate::{config::CONFIG, database::Database, models, utils};
use strum::IntoEnumIterator;

const SENTENCES: &str = include_str!("../../assets/default_tips.json");
const DEFAULT_CROPS_TYPES: &str = include_str!("../../assets/default_crop_types.json");

const ADMIN_PERMISSIONS: [models::PermissionType; 8] = [
    models::PermissionType::ViewDocuments,
    models::PermissionType::ViewDashboard,
    models::PermissionType::ViewDistribution,
    models::PermissionType::ViewUsers,
    models::PermissionType::ViewEdition,
    models::PermissionType::AddAccounts,
    models::PermissionType::EditAccounts,
    models::PermissionType::SendEmails,
];

const USERS_WITH_PERMISSIONS: [(models::RoleType, [models::PermissionType; 8]); 1] =
    [(models::RoleType::Admin, ADMIN_PERMISSIONS)];

pub async fn create_default_admin(database: &Database) -> anyhow::Result<i32> {
    match database
        .get_admin_by_email(CONFIG.admin_default_email.clone())
        .await
    {
        Ok(None) => {
            if let Ok(password) = utils::hash_password(&CONFIG.admin_default_password) {
                let admin = models::Admin {
                    id: None,
                    email: CONFIG.admin_default_email.clone(),
                    password,
                    role_id: models::RoleType::Admin.to_string(),
                };

                if let Ok(id) = database
                    .create_admin(admin, ADMIN_PERMISSIONS.iter().cloned().collect())
                    .await
                {
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

async fn apply_permissions_to_role(
    database: &Database,
    role: models::RoleType,
    permissions: &[models::PermissionType],
) -> anyhow::Result<()> {
    for permission in permissions.iter() {
        if let Ok(None) = database
            .get_role_permission(role.to_string(), permission.to_string())
            .await
        {
            database
                .create_role_permission(models::RolePermission {
                    id: None,
                    role_id: role.to_string(),
                    permission_id: permission.to_string(),
                })
                .await?;
        }
    }

    Ok(())
}

pub async fn associate_roles_permissions(database: &Database) -> anyhow::Result<()> {
    for (role, permissions) in USERS_WITH_PERMISSIONS.iter() {
        apply_permissions_to_role(database, *role, permissions).await?;
    }

    Ok(())
}

pub async fn create_default_tips(database: &Database) -> anyhow::Result<()> {
    let tips = serde_json::from_str::<Vec<String>>(SENTENCES)?;

    for tip in tips.iter() {
        database
            .create_tip(models::Tip {
                id: None,
                content: tip.clone(),
            })
            .await?;
    }

    Ok(())
}

pub async fn create_default_crop_types(database: &Database) -> anyhow::Result<()> {
    let crop_types = serde_json::from_str::<Vec<models::CropType>>(DEFAULT_CROPS_TYPES)?;

    for crop_type in crop_types.iter() {
        database
            .unsert_crop_types(crop_type.clone())
            .await?;
    }

    Ok(())
}

pub async fn setup(database: &Database) -> i32 {
    // Create the default roles
    create_default_roles(&database).await.unwrap();
    log::info!("Default roles created");

    // Create the default permissions
    create_default_permissions(&database).await.unwrap();
    log::info!("Default permissions created");

    // Associate roles with permissions
    associate_roles_permissions(&database).await.unwrap();
    log::info!("Roles associated with permissions");

    // Create the default admin
    let id = create_default_admin(&database).await.unwrap();

    // Create the default tips
    create_default_tips(&database).await.unwrap();
    log::info!("Default tips created");

    // Create the default crop types
    create_default_crop_types(&database).await.unwrap();
    log::info!("Default crop types created");

    id
}
