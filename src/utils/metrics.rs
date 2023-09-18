use std::{collections::HashMap, sync::{MutexGuard, Arc}, env};

use prometheus::{GaugeVec, HistogramVec, Registry, HistogramOpts, Opts};

use crate::types::druid_metrics::{DruidMetric, Returns};


static BASE_LABELS: [&'static str; 6] = [
    "service",
    "host",
    "duty_group",
    "cpu_name",
    "cpu_time",
    "id",
];

pub fn add_metric(
    data_gauge: &MutexGuard<'_, HashMap<String, GaugeVec>>,
    data_histogram: &MutexGuard<'_, HashMap<String, HistogramVec>>,
    metrics: Arc<&DruidMetric>,
    metric_name: &String,
    data_source: &String
) {
    let mut labels: HashMap<&str, &str> = HashMap::new();
    let mut labels2: HashMap<&str, String> = HashMap::new();

    for i in BASE_LABELS {
        let c = metrics.get(i).unwrap();

        let r = match c {
            Returns::String(t) => {
                t.clone().unwrap_or(String::from("---"))
            }
            Returns::Float(_) | Returns::Types(_) => String::from("---")
        };

        labels2.insert(
            i,
            r.clone()
        );
    };

    for i in labels2.keys() {
        labels.insert(*i, labels2.get(i).unwrap());
    }

    labels.insert("datasource", data_source.as_str());
    labels.insert("metric_name", metric_name.as_str());

    match data_gauge.get(metric_name) {
        Some(e) => {
            let _ = e.remove(&labels);
            e.with(&labels).set(metrics.value.unwrap_or(0.0));
        },
        None => ()
    };
    if env::var("USE_HISTOGRAM_METRICS").unwrap_or(String::from("false")) == "true" {
        match data_histogram.get(&(metric_name.clone() + "_histo")) {
            Some(e) => {
                let _ = e.remove(&labels);
                e.with(&labels).observe(metrics.value.unwrap_or(0.0));
            },
            None => ()
        };
    }
}

pub fn register_new_metric(
    metric_name: &String,
    registry: &MutexGuard<Registry>,
    data_gauge: &mut MutexGuard<'_, HashMap<String, GaugeVec>>,
    data_histogram: &mut MutexGuard<'_, HashMap<String, HistogramVec>>,
) {
    let mut labels = vec![
        "metric_name",
        "datasource",
    ];

    let mut c = BASE_LABELS.clone().to_vec();

    labels.append(&mut c);

    let metric_gauge = GaugeVec::new(
        Opts::new(
            metric_name.clone(),
            "Druid metric"
        ),
        &labels,
    ).expect("metric can be created");

    registry.register(Box::new(metric_gauge.clone())).expect("collector can be registered");
    data_gauge.insert(metric_name.clone(), metric_gauge);

    if env::var("USE_HISTOGRAM_METRICS").unwrap_or(String::from("false")) == "true" {
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
}