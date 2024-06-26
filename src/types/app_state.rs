use std::{collections::HashMap, sync::Mutex};

use prometheus::{Registry, GaugeVec, HistogramVec};

use super::metrics::Metrics;


pub struct AppState {
    pub registry: Mutex<Registry>,
    pub metrics_gauge: Mutex<HashMap<String, GaugeVec>>,
    pub metrics_histogram: Mutex<HashMap<String, HistogramVec>>,
    pub cw: Mutex<Option<aws_sdk_cloudwatch::Client>>,
    pub dispatchers: Mutex<Vec<String>>,
    pub metrics: Mutex<Metrics>,
}