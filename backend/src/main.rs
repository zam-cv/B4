use dotenv::dotenv;

mod app;
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
    // Load the environment variables
    dotenv().ok();

    // Initialize the logger
    env_logger::init();

    // Start the application
    app::app().await
}
