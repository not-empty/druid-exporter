use actix_web::{post, web, Responder};

use crate::{adapters::api::druid::controllers::druid::druid_controller, types::{app_state::AppState, druid::metrics::DruidMetric}};


#[post("/druid")]
pub async fn druid_handler(
    body: web::Json<Vec<DruidMetric>>,
    state: web::Data<AppState>
) -> impl Responder {
    druid_controller(body, state).await
}