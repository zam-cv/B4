use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;

pub struct Config {
  pub address: String,
  pub secret_key: String,
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
          secret_key: env::var("SECRET_KEY").expect("SECRET_KEY must be set"),
      }
  };
}

pub const TOKEN_EXPIRATION_TIME: usize = 60 * 60 * 24 * 15; // 16 days