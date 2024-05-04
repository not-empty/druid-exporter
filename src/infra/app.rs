use std::{collections::HashMap, env};

use actix_web::{App, HttpServer, web, middleware::Logger};
use aws_config::BehaviorVersion;
use aws_sdk_cloudwatch::config::{Credentials, SharedCredentialsProvider};
use prometheus::Registry;

use crate::adapters::api::{druid::router::druid_handler, health::router::health_handler, metrics::router::metrics_handler};
use crate::types::app_state::AppState;
use crate::utils::file::read_config_yaml;


fn get_dispatchers() -> Vec<String> {
    Vec::from_iter(
        env::var("DISPATCHERS")
            .unwrap_or(String::from("prometheus"))
            .split(",")
            .map(str::to_string)
    )
}

async fn get_cw_client() -> Option<aws_sdk_cloudwatch::Client> {
    let d = get_dispatchers();

    if !d.contains(&String::from("cloudwatch")) {
        return None;
    }

    let config = aws_config::defaults(BehaviorVersion::latest())
        .region("sa-east-1")
        .credentials_provider(SharedCredentialsProvider::new(Credentials::new(
            env::var("AWS_KEY").unwrap_or(String::from("")),
            env::var("AWS_SECRET").unwrap_or(String::from("")),
            None,
            None,
            "cloudwatch"
        )))
        .load().await;

    Some(aws_sdk_cloudwatch::Client::new(&config).into())
}

pub async fn start_app() -> std::io::Result<()> {
    let data = web::Data::new(AppState {
        registry: Registry::new().into(),
        metrics_gauge: HashMap::new().into(),
        metrics_histogram: HashMap::new().into(),
        cw: get_cw_client().await.into(),
        dispatchers: get_dispatchers().into(),
        metrics: read_config_yaml().into(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::PayloadConfig::new(20971520))
            .app_data(data.clone())
            .service(health_handler)
            .service(druid_handler)
            .service(metrics_handler)
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 7080))?
    .run()
    .await
}