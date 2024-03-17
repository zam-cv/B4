use crate::{
    bank, config::CONFIG, database::Database, middlewares, routes, socket, socket::server::Server,
};
use actix_cors::Cors;
use actix_files as fs;
use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_lab::middleware::from_fn;
use ip2location::DB;
use std::sync::{atomic::AtomicUsize, Arc, Mutex};
use tokio::sync::broadcast;

const IPV6BIN: &str = "assets/IP2LOCATION-LITE-DB5.IPV6.BIN";

pub async fn app() -> std::io::Result<()> {
    let (viewer_tx, _) = broadcast::channel::<()>(10);
    let viewer_tx_clone = viewer_tx.clone();

    let bank = bank::Bank::new();
    log::info!("Bank created");

    let location_db = Arc::new(Mutex::new(DB::from_file(IPV6BIN).unwrap()));
    log::info!("Location database connected");

    let database = Database::new();
    log::info!("Database connected");

    let (mut socket_server, server_tx) = Server::new(bank, database.clone());
    tokio::spawn(async move { socket_server.run().await });
    log::info!("Socket server started");

    // Create a counter for the number of visitors
    let visitor_count = Arc::new(AtomicUsize::new(0));

    let server = HttpServer::new(move || {
        let cors = Cors::permissive().supports_credentials();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(location_db.clone()))
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(server_tx.clone()))
            .app_data(web::Data::new(viewer_tx_clone.clone()))
            .app_data(web::Data::new(visitor_count.clone()))
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
                            .service(routes::user::auth::register)
                            .service(routes::signout),
                    )
                    .service(
                        web::scope("/admin")
                            .service(
                                web::scope("/auth")
                                    .service(routes::admin::signin)
                                    .service(routes::admin::register)
                                    .service(routes::signout)
                                    .service(
                                        web::scope("")
                                            .wrap(from_fn(middlewares::admin_auth))
                                            .service(routes::admin::auth),
                                    ),
                            )
                            .service(
                                web::scope("")
                                    .wrap(from_fn(middlewares::admin_auth))
                                    .service(
                                        web::scope("/user")
                                            .service(routes::user::info::get_user_statistics)
                                            .service(routes::user::info::get_user),
                                    )
                                    .service(
                                        web::scope("/users").service(routes::user::info::get_users),
                                    )
                                    .service(
                                        web::scope("/player").service(routes::player::get_player),
                                    )
                                    .service(
                                        web::scope("/players")
                                            .service(routes::player::get_players_count),
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
