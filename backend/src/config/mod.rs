use lazy_static::lazy_static;
use std::env;

pub mod database;
pub mod ssl;

pub struct Config {
    pub mode: String,
    pub port: String,
    pub host: String,
    pub address: String,
    pub user_secret_key: String,
    pub admin_secret_key: String,
    pub database_url: String,
    pub ipinfo_token: String,
    pub admin_default_email: String,
    pub admin_default_password: String,
    pub smtp_host: String,
    pub smtp_username: String,
    pub smtp_password: String,
    pub sender: String,
}

lazy_static! {
    pub static ref CONFIG: Config = {
        let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");

        Config {
            mode: env::var("MODE").expect("MODE must be set"),
            port: env::var("PORT").expect("PORT must be set"),
            host: env::var("HOST").expect("HOST must be set"),
            address: format!(
                "{}:{}",
                env::var("HOST").expect("HOST must be set"),
                env::var("PORT").expect("PORT must be set")
            ),
            user_secret_key: env::var("USER_SECRET_KEY").expect("USER_SECRET_KEY must be set"),
            admin_secret_key: env::var("ADMIN_SECRET_KEY").expect("ADMIN_SECRET_KEY must be set"),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            ipinfo_token: env::var("IPINFO_TOKEN").expect("IPINFO_TOKEN must be set"),
            admin_default_email: env::var("ADMIN_DEFAULT_EMAIL")
                .expect("ADMIN_DEFAULT_EMAIL must be set"),
            admin_default_password: env::var("ADMIN_DEFAULT_PASSWORD")
                .expect("ADMIN_DEFAULT_PASSWORD must be set"),
            smtp_host: env::var("SMTP_HOST").expect("SMTP_HOST must be set"),
            smtp_username: smtp_username.clone(),
            smtp_password: env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set"),
            sender: format!("Verqor <{}>", smtp_username),
        }
    };
}

// User constants
pub const TOKEN_EXPIRATION_TIME: usize = 60 * 60 * 24 * 15; // 15 days

// Valores para ponderar max change
pub const CASH_WEIGHT: i32 = 60;
pub const VERQOR_WEIGHT: i32 = 30;
pub const COYOTE_WEIGHT: i32 = 10;

// Game constants
pub const INTEREST_PERCENTAGE_COYOTE: f64 = 0.2;
pub const INTEREST_PERCENTAGE_VERQOR: f64 = 0.05;
pub const REVENUE_PERCENTAGE: f64 = 0.2;
pub const TOP_PLAYERS: i64 = 5;
pub const CREDIT_LIMIT: i32 = -5000;
pub const INITIAL_TIME: i32 = 0;
pub const INITIAL_TIME_IN_GAME: f64 = 0.0;
pub const INITIAL_CYCLE: i32 = 0;
pub const INITIAL_SCORE: f64 = 0.0;
pub const INITIAL_BALANCE: i32 = 0;
pub const INITIAL_BALANCE_CASH: i32 = 500000;
pub const INITIAL_PLOTS: i32 = 4;
pub const INITIAL_MAX_CHANGE: i32 = CASH_WEIGHT * INITIAL_BALANCE_CASH
    + VERQOR_WEIGHT * INITIAL_BALANCE
    + COYOTE_WEIGHT * INITIAL_BALANCE;