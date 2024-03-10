use crate::{
    database::Database,
    socket::server::{Command, Server},
};
use actix_files as fs;
use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_lab::middleware::from_fn;
use config::CONFIG;
use tokio::sync::broadcast;

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
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    let (viewer_tx, _) = broadcast::channel::<Command>(10);
    let viewer_tx_clone = viewer_tx.clone();

    let bank = bank::Bank::new();
    log::info!("Bank created");

    let database = Database::new();
    log::info!("Database connected");

    let (mut socket_server, server_tx) = Server::new(bank, database.clone());
    tokio::spawn(async move { socket_server.run(viewer_tx).await });
    log::info!("Socket server started");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(server_tx.clone()))
            .app_data(web::Data::new(viewer_tx_clone.clone()))
            .route(
                "/ws/",
                web::get()
                    .to(socket::server_index)
                    // Wrap the websocket route with the user_auth middleware
                    .wrap(from_fn(middlewares::user_auth)),
            )
            .route(
                "/viewer/",
                web::get()
                    .to(socket::viewer_index)
                    .wrap(from_fn(middlewares::admin_auth)),
            )
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/auth")
                            .service(routes::user::auth::signin)
                            .service(routes::user::auth::register),
                    )
                    .service(
                        web::scope("/admin")
                            .service(
                                web::scope("/auth")
                                    .service(routes::admin::signin)
                                    .service(routes::admin::register),
                            )
                            .service(
                                web::scope("")
                                    .wrap(from_fn(middlewares::admin_auth))
                                    .service(
                                        web::scope("/user")
                                            .service(routes::user::info::get_user_statistics)
                                            .service(routes::user::info::get_user)
                                    )
                                    .service(
                                        web::scope("/users")
                                            .service(routes::user::info::get_users),
                                    )
                                    .service(
                                        web::scope("/player")
                                            .service(routes::player::get_player),
                                    )
                                    .service(
                                        web::scope("/data")
                                            .service(routes::admin::create_crop_type),
                                    ),
                            ),
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
