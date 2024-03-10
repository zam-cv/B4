use crate::{
    config::{self, CONFIG},
    database::Database,
    models,
    routes::{signin, Credentials, Response, Status},
    utils,
};
use actix_web::{error, post, web, HttpRequest, Responder, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use woothee::parser::Parser;

signin!(get_user_by_email, CONFIG.user_secret_key);

#[post("/register")]
pub async fn register(
    req: HttpRequest,
    database: web::Data<Database>,
    mut user: web::Json<models::User>,
) -> Result<impl Responder> {
    if let Ok(hash) = utils::get_hash!(user.password) {
        if let Ok(None) = database.get_user_by_email(user.username.clone()).await {
            let parser = Parser::new();

            let ip = req.peer_addr().map(|addr| addr.ip().to_string());
            log::debug!("IP: {:?}", ip);
            let os = if let Some(user_agent) = req.headers().get("user-agent") {
                if let Ok(user_agent) = user_agent.to_str() {
                    parser.parse(user_agent).map(|ua| ua.os.to_string())
                } else {
                    None
                }
            } else {
                None
            };

            let player_id = database
                .create_player(models::Player {
                    id: None,
                    current_cycle: 0,
                    current_score: config::INITIAL_SCORE,
                    current_balance: config::INITIAL_BALANCE,
                    max_plots: config::INITIAL_MAX_PLOTS,
                })
                .await
                .map_err(|_| error::ErrorBadRequest("Failed"))?;

            user.id = None;
            user.ip = ip;
            user.password = hash.to_string();
            user.os = os;
            user.player_id = player_id;

            let user_id = database
                .create_user(user.into_inner())
                .await
                .map_err(|_| error::ErrorBadRequest("Failed"))?;

            if let Ok(token) = utils::create_token(&CONFIG.user_secret_key, user_id) {
                return Ok(web::Json(Response {
                    message: Status::Success,
                    payload: Some(Credentials { token }),
                }));
            }
        }

        return Ok(web::Json(Response {
            message: Status::Incorrect("Email already exists"),
            payload: None,
        }));
    }

    Err(actix_web::error::ErrorBadRequest("Failed"))
}
