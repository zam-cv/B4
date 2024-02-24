use actix_files as fs;
use actix_web::{web, App, HttpServer};
use config::CONFIG;
use database::Database;

mod config;
mod database;
mod models;
mod schema;
mod routes;
mod socket;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database = Database::new();
    println!("Server running at {}", CONFIG.address);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.clone()))
            .route("/ws/", web::get().to(socket::index))
            .service(
                web::scope("/api").service(
                    web::scope("/admin")
                        .service(routes::login_admin)
                        .service(routes::register_admin),
                ),
            )
            .service(
                fs::Files::new("/", "../page/")
                    .show_files_listing()
                    .index_file("index.html"),
            )
    })
    .bind(&CONFIG.address)?
    .run()
    .await
}
