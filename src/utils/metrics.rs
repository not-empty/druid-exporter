use std::{collections::HashMap, sync::{MutexGuard, Arc}};

use prometheus::{GaugeVec, HistogramVec, Registry, HistogramOpts, Opts};

use crate::types::druid_metrics::DruidMetric;

pub fn add_metric(
    data_gauge: &MutexGuard<'_, HashMap<String, GaugeVec>>,
    data_histogram: &MutexGuard<'_, HashMap<String, HistogramVec>>,
    metrics: Arc<&DruidMetric>,
    metric_name: &String,
    data_source: &String
) {
    let mut labels: HashMap<&str, &str> = HashMap::new();

    let duty = metrics.duty_group.to_owned().unwrap_or(String::from("---"));
    let host = metrics.host.clone().unwrap_or(String::from("---"));
    let service = metrics.service.clone().unwrap_or(String::from("---"));
    let cpu_time = metrics.cpu_time.clone().unwrap_or(String::from("---"));
    let cpu_name = metrics.cpu_name.clone().unwrap_or(String::from("---"));

    labels.insert("metric_name", metric_name.as_str());
    labels.insert("service", service.as_str());
    labels.insert("datasource", data_source.as_str());
    labels.insert("host", host.as_str());
    labels.insert("duty_group", duty.as_str());
    labels.insert("cpu_time", cpu_time.as_str());
    labels.insert("cpu_name", cpu_name.as_str());

    match data_gauge.get(metric_name) {
        Some(e) => {
            let _ = e.remove(&labels);
            e.with(&labels).set(metrics.value.unwrap_or(0.0));
        },
        None => println!("ad")
    };

    match data_histogram.get(&(metric_name.clone() + "_histo")) {
        Some(e) => {
            let _ = e.remove(&labels);
            e.with(&labels).observe(metrics.value.unwrap_or(0.0));
        },
        None => println!("ad")
    };
}


pub fn register_new_metric(
    metric_name: &String,
    registry: &MutexGuard<Registry>,
    data_gauge: &mut MutexGuard<'_, HashMap<String, GaugeVec>>,
    data_histogram: &mut MutexGuard<'_, HashMap<String, HistogramVec>>,
) {
    let labels = &vec![
        "metric_name",
        "service",
        "datasource",
        "host",
        "duty_group",
        "cpu_name",
        "cpu_time",
    ];

    let metric_gauge = GaugeVec::new(
        Opts::new(
            metric_name.clone(),
            "Druid metric"
        ),
        &labels,
    ).expect("metric can be created");

    registry.register(Box::new(metric_gauge.clone())).expect("collector can be registered");
    data_gauge.insert(metric_name.clone(), metric_gauge);

    let metric_histogram = HistogramVec::new(
        HistogramOpts::new(
            metric_name.clone() + "_histo",
            "Druid metric histo"
        ),
        &labels,
    ).expect("metric can be created");

    registry.register(Box::new(metric_histogram.clone())).expect("collector can be registered");
    data_histogram.insert(metric_name.clone() + "_histo", metric_histogram);
}