use actix_web::{web, HttpResponse, Responder};

use crate::{application::use_cases::druid::send_prometheus_metrics::PrometheusStrategy, types::{app_state::AppState, druid::{dispatcher::DispatcherNavigator, metrics::DruidMetric}}};


pub async fn druid_controller(
    body: web::Json<Vec<DruidMetric>>,
    state: web::Data<AppState>,
) -> impl Responder {
    let metrics = body.as_slice();

    let navigator = DispatcherNavigator::new(PrometheusStrategy);
    let prom = navigator.send(metrics, state.clone());

    prom.await;

    HttpResponse::Ok().body("ok")
}
