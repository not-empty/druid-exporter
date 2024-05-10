use actix_web::{get, HttpResponse, Responder};


#[get("/health")]
pub async fn health_handler() -> impl Responder {
    HttpResponse::Ok().body("ok")
}