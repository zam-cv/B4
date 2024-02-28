use actix::prelude::*;
use actix_files as fs;
use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_lab::middleware::from_fn;
use config::CONFIG;
use database::Database;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let bank = bank::Bank::new();
    log::info!("Bank created");

    let database = Database::new();
    log::info!("Database connected");

    let socket_server = socket::server::Server::new(bank, database.clone()).start();
    log::info!("Socket server started");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(socket_server.clone()))
            .route(
                "/ws/",
                web::get()
                    .to(socket::index)
                    // Wrap the websocket route with the user_auth middleware
                    .wrap(from_fn(middlewares::user_auth)),
            )
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/public")
                            .service(
                                web::scope("/admin")
                                    .service(routes::login_admin)
                                    .service(routes::register_admin),
                            )
                            .service(
                                web::scope("/user")
                                    .service(routes::login_user)
                                    .service(routes::register_user),
                            ),
                    )
                    .service(
                        web::scope("/private")
                            .wrap(from_fn(middlewares::admin_auth))
                            .service(web::scope("/admin").service(routes::get_users)),
                    ),
            )
            .service(
                fs::Files::new("/", "../page/")
                    .show_files_listing()
                    .index_file("index.html"),
            )
            .wrap(Logger::default())
    })
    .bind(&CONFIG.address)?;

    log::info!("Server running at http://{}", &CONFIG.address);
    server.run().await
}
