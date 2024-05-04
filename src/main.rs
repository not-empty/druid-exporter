mod types;
mod utils;
mod adapters;
mod application;

use std::{collections::HashMap, env};

use actix_web::{App, HttpServer, web, middleware::Logger};
use aws_config::BehaviorVersion;
use aws_sdk_cloudwatch::config::{Credentials, SharedCredentialsProvider};
use dotenv::dotenv;
use env_logger::Env;
use prometheus::Registry;
use adapters::api::{druid::router::druid_handler, health::router::health_handler, metrics::router::metrics_handler};
use types::app_state::AppState;
use utils::file::read_file_icon;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    read_file_icon();

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

    let dispatchers = Vec::from_iter(
        env::var("DISPATCHERS")
            .unwrap_or(String::from("prometheus"))
            .split(",")
            .map(str::to_string)
    );

    let data = web::Data::new(AppState {
        registry: Registry::new().into(),
        metrics_gauge: HashMap::new().into(),
        metrics_histogram: HashMap::new().into(),
        cw: aws_sdk_cloudwatch::Client::new(&config).into(),
        dispatchers: dispatchers.into(),
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