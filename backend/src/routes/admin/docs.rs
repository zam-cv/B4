use actix_web::{get, web, HttpResponse, Responder};

#[get("/docs/swagger.json")]
pub async fn api(doc_json: web::Data<String>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(doc_json.as_ref().clone())
}
