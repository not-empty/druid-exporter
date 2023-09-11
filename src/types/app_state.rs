use std::{collections::HashMap, sync::Mutex};

use prometheus::{Registry, GaugeVec, HistogramVec};

pub struct AppState {
    pub registry: Mutex<Registry>,
    pub metrics_gauge: Mutex<HashMap<String, GaugeVec>>,
    pub metrics_histogram: Mutex<HashMap<String, HistogramVec>>,
}