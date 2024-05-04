use futures::future::join;
use actix_web::{web, HttpResponse, Responder};

use crate::{application::use_cases::druid::{send_cloudwatch_metrics::cloudwatch_publisher, send_prometheus_metrics::prometheus_publisher}, types::{app_state::AppState, druid_metrics::DruidMetric}};


pub async fn druid_controller(
    body: web::Json<Vec<DruidMetric>>,
    state: web::Data<AppState>,
) -> impl Responder {
    let metrics = body.as_slice();

    join(
        async {
            prometheus_publisher(
                metrics,
                state.clone(),
            )
        },
        cloudwatch_publisher(
            metrics,
            state.clone(),
        ),
    ).await;

    HttpResponse::Ok().body("ok")
}