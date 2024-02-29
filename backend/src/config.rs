use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;

pub struct Config {
    pub address: String,
    pub user_secret_key: String,
    pub admin_secret_key: String,
    pub database_url: String,
}

lazy_static! {
    pub static ref CONFIG: Config = {
        dotenv().ok();

        Config {
            address: format!(
                "{}:{}",
                env::var("HOST").expect("HOST must be set"),
                env::var("PORT").expect("PORT must be set")
            ),
            user_secret_key: env::var("USER_SECRET_KEY").expect("USER_SECRET_KEY must be set"),
            admin_secret_key: env::var("ADMIN_SECRET_KEY").expect("ADMIN_SECRET_KEY must be set"),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        }
    };
}

pub const TOKEN_EXPIRATION_TIME: usize = 60 * 60 * 24 * 15; // 15 days
pub const INITIAL_BALANCE_CASH: i32 = 1000;
pub const INITIAL_BALANCE_VERQOR: i32 = 1000;
pub const INITIAL_BALANCE_COYOTE: i32 = 1000;