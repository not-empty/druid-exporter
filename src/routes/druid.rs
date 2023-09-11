use std::sync::Arc;

use actix_web::{post, HttpResponse, Responder, web};

use crate::{types::{app_state::AppState, druid_metrics::{DruidMetric, DataSourceTypes}}, utils::metrics::{add_metric, register_new_metric}};


#[post("/druid")]
pub async fn druid_handler(body: web::Json<Vec<DruidMetric>>, data: web::Data<AppState>) -> impl Responder {
    let registry = data.registry.lock().unwrap();
    let mut metrics_gauge = data.metrics_gauge.lock().unwrap();
    let mut metrics_histogram = data.metrics_histogram.lock().unwrap();

    for i in body.iter() {
        let data = Arc::new(i);
        let druid_metric = data.metric.clone().unwrap_or(String::default());

        if druid_metric == String::default() {
            continue;
        }

        let metric_name = String::from("druid_expo_")
            + &druid_metric.
                clone().
                to_string().
                replace("/", "_");

        if !metrics_gauge.contains_key(&metric_name) {
            register_new_metric(
                &metric_name,
                &registry,
                &mut metrics_gauge,
                &mut metrics_histogram
            );
        }

        let sources = match data.data_source.clone() {
            DataSourceTypes::String(data_source) => vec![data_source],
            DataSourceTypes::Vec(data_sources) => data_sources,
        };

        for j in sources {
            add_metric(
                &metrics_gauge,
                &metrics_histogram,
                data.clone(),
                &metric_name,
                &j
            )
        }
    }

    HttpResponse::Ok().body("ok")
}

