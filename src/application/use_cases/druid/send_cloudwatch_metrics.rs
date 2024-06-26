use std::sync::Arc;

use actix_web::web;
use aws_sdk_cloudwatch::types::MetricDatum;
use std::sync::MutexGuard;

use crate::{types::{app_state::AppState, druid::{dispatcher::DispatcherStrategy, metrics::DruidMetric}, metrics::Metrics}, utils::metrics::{add_cw_metric, check_allowed_metric, transform_metric_name}};


pub struct CloudwatchStrategy;

impl CloudwatchStrategy {
    fn get_metric(
        &self,
        metric: &DruidMetric,
        metrics_config: &MutexGuard<Metrics>
    ) -> Option<aws_sdk_cloudwatch::types::MetricDatum> {
        let data = Arc::new(metric);
        let druid_metric = data.metric.clone().unwrap_or(String::default());

        if druid_metric == String::default() {
            return None;
        }

        let metric_name = transform_metric_name(druid_metric.clone());

        if !check_allowed_metric(metrics_config, metric_name.clone()) {
            return None;
        }

        Some(add_cw_metric(
            Arc::clone(&data),
            &metric_name,
        ))
    }
}

impl DispatcherStrategy for CloudwatchStrategy {
    async fn send_event(
        &self,
        metrics: &[DruidMetric],
        state: web::Data<AppState>,
    ) {
        if !state.dispatchers.lock().unwrap().contains(&String::from("cloudwatch")){
            return;
        }

        let mut metric_data: Vec<MetricDatum> = Vec::new();
        let cw = <std::option::Option<aws_sdk_cloudwatch::Client> as Clone>::clone(&state.cw.lock().unwrap()).unwrap();
        let metrics_config = state.metrics.lock().unwrap();

        for i in metrics {
            match self.get_metric(i, &metrics_config) {
                Some(e) => metric_data.push(e),
                None => continue,
            };
        }

        let r = cw.put_metric_data()
            .namespace("druid_expo_metrics".to_string())
            .set_metric_data(Some(metric_data))
            .send()
            .await;

        match r {
            Err(_) => {
                log::error!("Error to send metric do CW, Please verify your credentials");
            },
            _ => {}
        };
    }
}
