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

signin!(get_user_by_username, CONFIG.user_secret_key);

#[post("/register")]
pub async fn register(
    req: HttpRequest,
    database: web::Data<Database>,
    info: web::Json<models::User>,
) -> Result<impl Responder> {
    if let Ok(hash) = utils::get_hash!(info.password) {
        if let Ok(None) = database.get_user_by_username(info.username.clone()).await {
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

            let id = database
                .create_user(models::User {
                    id: None,
                    username: info.username.clone(),
                    password: hash.to_string(),
                    balance_cash: config::INITIAL_BALANCE_CASH,
                    balance_verqor: config::INITIAL_BALANCE_VERQOR,
                    balance_coyote: config::INITIAL_BALANCE_COYOTE,
                    current_day: chrono::Local::now().naive_local(),
                    max_sections: config::INITIAL_MAX_SECTIONS,
                    ip,
                    os,
                })
                .await
                .map_err(|_| error::ErrorBadRequest("Failed"))?;

            if let Ok(token) = utils::create_token(&CONFIG.user_secret_key, id) {
                return Ok(web::Json(Response {
                    message: Status::Success,
                    payload: Some(Credentials { token }),
                }));
            }
        }

        return Ok(web::Json(Response {
            message: Status::Incorrect("Username already exists"),
            payload: None,
        }));
    }

    Err(actix_web::error::ErrorBadRequest("Failed"))
}
