use crate::{models, routes};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(description = "This is the API documentation for the backend server"),
    paths(
        routes::auth::signin,
        routes::auth::register,
        routes::auth::signout,
        routes::auth::auth,
        routes::admin::auth::auth,
        routes::admin::auth::signin,
        routes::admin::auth::signout,
        routes::admin::auth::register,
        routes::admin::permissions::get_permissions,
        routes::admin::permissions::get_permission_types,
        routes::admin::permissions::get_permissions_by_admin_id,
        routes::admin::permissions::add_permission,
        routes::admin::permissions::delete_permission,
        routes::admin::admins::delete_admin,
        routes::admin::admins::get_admins,
        routes::admin::data::create_crop_type,
        routes::admin::player::get_player,
        routes::admin::players::get_players_count,
        routes::admin::user::get_user,
        routes::admin::user::get_user_statistics,
        routes::admin::users::get_users,
        routes::admin::users::get_user_types,
        routes::admin::users::get_user_count_by_type
    ),
    components(schemas(
        routes::UserCredentials,
        routes::AdminCredentials,
        models::User,
        models::Gender,
        models::UserType,
        models::Admin,
        routes::AdminInfo,
        models::CropType,
        models::Player,
        models::StatisticsSample,
        models::PermissionType,
        routes::admin::permissions::PermissionPayload
    ))
)]
pub struct ApiDoc;
