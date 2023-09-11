use actix_web::{get, HttpResponse, Responder, web};
use prometheus::Encoder;

use crate::types;


#[get("/metrics")]
pub async fn metrics_handler(data: web::Data<types::app_state::AppState>) -> impl Responder {
    let encoder = prometheus::TextEncoder::new();
    let registry = data.registry.lock().unwrap();

    let mut buffer = vec![];

    encoder.encode(&registry.gather(), &mut buffer)
        .expect("Failed to encode metrics");

    let response = String::from_utf8(buffer.clone()).expect("Failed to convert bytes to string");
    buffer.clear();

    HttpResponse::Ok().body(response)
}