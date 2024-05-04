use std::sync::Arc;

use actix_web::web;

use crate::{types::{app_state::AppState, druid_metrics::{DataSourceTypes, DruidMetric}}, utils::metrics::{add_metric, register_new_metric, transform_metric_name}};


pub fn prometheus_publisher(
    metrics: &[DruidMetric],
    state: web::Data<AppState>,
) {
    if !state.dispatchers.lock().unwrap().contains(&String::from("prometheus")){
        return;
    }

    let registry = state.registry.lock().unwrap();
    let mut metrics_gauge = state.metrics_gauge.lock().unwrap();
    let mut metrics_histogram = state.metrics_histogram.lock().unwrap();

    for i in metrics {
        let data = Arc::new(i);
        let druid_metric = data.metric.clone().unwrap_or(String::default());

        if druid_metric == String::default() {
            continue;
        }

        let metric_name = transform_metric_name(druid_metric.clone());

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
                Arc::clone(&data),
                &metric_name,
                &j
            );
        }
    }
}