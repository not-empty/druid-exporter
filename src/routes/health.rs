use actix_web::{Responder, HttpResponse, get};


#[get("/health")]
pub async fn health_handler() -> impl Responder {
    HttpResponse::Ok().body("ok")
}