use actix_web::{get, Responder, web};

use crate::{adapters::api::metrics::controllers::metrics::metrics_controller, types};


#[get("/metrics")]
pub async fn metrics_handler(data: web::Data<types::app_state::AppState>) -> impl Responder {
    metrics_controller(data)
}