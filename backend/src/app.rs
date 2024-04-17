use crate::{
    bank,
    config::{self, CONFIG},
    database::Database,
    docs::ApiDoc,
    middlewares, routes, socket,
    socket::server::Server,
};
use actix_cors::Cors;
use actix_files as fs;
use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_lab::middleware::from_fn;
use ip2location::DB;
use std::sync::{atomic::AtomicUsize, Arc, Mutex};
use tokio::sync::broadcast;
use utoipa::OpenApi;

const IPV6BIN: &str = "assets/IP2LOCATION-LITE-DB5.IPV6.BIN";

pub async fn app() -> std::io::Result<()> {
    // Create a channel for the viewer
    let (viewer_tx, _) = broadcast::channel::<()>(10);
    let viewer_tx_clone = viewer_tx.clone();

    // Create the bank
    let bank = bank::Bank::new();
    log::info!("Bank created");

    // Load the location database
    let location_db = if let Ok(db) = DB::from_file(IPV6BIN) {
        Some(Arc::new(Mutex::new(db)))
    } else {
        log::warn!("Failed to load location database, using empty database");
        None
    };
    log::info!("Location database connected");

    // Create the database
    let database = Database::new();
    log::info!("Database connected");

    // Setup the database
    let default_admin_id = config::database::setup(&database).await;

    // Create the socket server
    let (mut socket_server, server_tx) = Server::new(bank, database.clone());
    tokio::spawn(async move { socket_server.run().await });
    log::info!("Socket server started");

    // Create a counter for the number of visitors
    let visitor_count = Arc::new(AtomicUsize::new(0));

    // Generate the OpenAPI documentation
    let openapi = ApiDoc::openapi();
    let doc_json = openapi.to_json().unwrap();
    log::info!("OpenAPI documentation generated");

    // Create the server
    let server = HttpServer::new(move || {
        // Create the CORS middleware
        let cors = Cors::permissive().supports_credentials();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(default_admin_id))
            .app_data(web::Data::new(location_db.clone()))
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(server_tx.clone()))
            .app_data(web::Data::new(viewer_tx_clone.clone()))
            .app_data(web::Data::new(visitor_count.clone()))
            .app_data(web::Data::new(doc_json.clone()))
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
                            .service(routes::auth::signin)
                            .service(routes::auth::register)
                            .service(routes::auth::signout)
                            .service(
                                web::scope("")
                                    .wrap(from_fn(middlewares::user_auth))
                                    .service(routes::auth::auth),
                            ),
                    )
                    .service(
                        web::scope("/admin")
                            .service(
                                web::scope("/auth")
                                    .service(routes::admin::auth::signin)
                                    .service(routes::admin::auth::signout)
                                    .service(
                                        web::scope("")
                                            .wrap(from_fn(middlewares::admin_auth))
                                            .service(routes::admin::auth::register)
                                            .service(routes::admin::auth::auth),
                                    ),
                            )
                            .service(
                                web::scope("")
                                    .wrap(from_fn(middlewares::admin_auth))
                                    .service(routes::admin::docs::api)
                                    .service(
                                        web::scope("/admins")
                                            .service(routes::admin::admins::delete_admin)
                                            .service(routes::admin::admins::get_admins),
                                    )
                                    .service(
                                        web::scope("/user")
                                            .service(routes::admin::user::get_user_statistics)
                                            .service(routes::admin::user::get_user),
                                    )
                                    .service(
                                        web::scope("/users")
                                            .service(routes::admin::users::get_users)
                                            .service(routes::admin::users::get_user_types)
                                            .service(routes::admin::users::get_user_count_by_type)
                                            .service(routes::admin::users::get_user_genders)
                                            .service(routes::admin::users::get_user_count_by_gender)
                                            .service(routes::admin::users::get_user_count_by_age_range)
                                            .service(routes::admin::users::get_user_locations_by_type)
                                            .service(routes::admin::users::get_average_age)
                                            .service(routes::admin::users::get_average_sessions)
                                            .service(routes::admin::users::get_average_time_in_game),
                                    )
                                    .service(
                                        web::scope("/player")
                                            .service(routes::admin::player::get_player),
                                    )
                                    .service(
                                        web::scope("/players")
                                            .service(routes::admin::players::get_players_count)
                                            .service(routes::admin::players::get_average_time_in_game),
                                    )
                                    .service(
                                        web::scope("/data")
                                            .service(routes::admin::data::create_crop_type)
                                            .service(routes::admin::data::get_tips)
                                            .service(routes::admin::data::create_tip)
                                            .service(routes::admin::data::update_tip)
                                            .service(routes::admin::data::delete_tip)
                                    )
                                    .service(
                                        web::scope("/permissions")
                                            .service(routes::admin::permissions::get_permissions)
                                            .service(routes::admin::permissions::get_permission_types)
                                            .service(
                                                routes::admin::permissions::get_permissions_by_admin_id,
                                            )
                                            .service(routes::admin::permissions::add_permission)
                                            .service(routes::admin::permissions::delete_permission),
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
    });

    // SSL configuration
    let server = if let Ok(builder) = config::ssl::get_ssl_acceptor() {
        log::info!("SSL configuration loaded");
        log::info!("Server running at https://{}", &CONFIG.address);
        server.bind_openssl(&CONFIG.address, builder)?
    } else {
        log::warn!("Failed to load SSL configuration, using insecure connection");
        log::info!("Server running at http://{}", &CONFIG.address);
        server.bind(&CONFIG.address)?
    };

    server.run().await
}
