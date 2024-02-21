use actix_files as fs;
use actix_web::{App, HttpServer};

pub const HOST: &str = "0.0.0.0";
pub const PORT: &str = "8080";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at {}:{}", HOST, PORT);
    HttpServer::new(|| {
        App::new().service(
            fs::Files::new("/", "../page/")
                .show_files_listing()
                .index_file("index.html"),
        )
    })
    .bind(format!("{}:{}", HOST, PORT))?
    .run()
    .await
}
