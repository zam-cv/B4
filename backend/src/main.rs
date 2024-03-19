mod app;
#[allow(dead_code, unused_imports, private_interfaces)]
mod bank;
mod config;
mod database;
mod middlewares;
mod models;
mod routes;
mod schema;
mod socket;
mod utils;
mod docs;

#[cfg(test)]
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    app::app().await
}
