use actix_web::{ HttpResponse, Responder, web};
use prometheus::Encoder;

use crate::types;


pub fn metrics_controller(state: web::Data<types::app_state::AppState>) -> impl Responder {
    let encoder = prometheus::TextEncoder::new();
    let registry = state.registry.lock().unwrap();

    let mut buffer = vec![];

    encoder
        .encode(&registry.gather(), &mut buffer)
        .expect("Failed to encode metrics");

    let response = String::from_utf8(buffer.clone())
        .expect("Failed to convert bytes to string");

    buffer.clear();

    HttpResponse::Ok().body(response)
}