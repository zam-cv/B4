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
        routes::admin::data::create_crop_type,
        routes::admin::player::get_player,
        routes::admin::players::get_players_count,
        routes::admin::user::get_user,
        routes::admin::user::get_user_statistics,
        routes::admin::users::get_users,
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
        models::StatisticsSample
    ))
)]
pub struct ApiDoc;
