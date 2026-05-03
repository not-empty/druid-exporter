use std::collections::HashMap;

use actix_web::{App, HttpServer, web, middleware::Logger};
use prometheus::Registry;

use crate::adapters::api::{druid::router::druid_handler, health::router::health_handler, metrics::router::metrics_handler};
use crate::types::app_state::AppState;
use crate::utils::file::read_config_yaml;

pub async fn start_app() -> std::io::Result<()> {
    let data = web::Data::new(AppState {
        registry: Registry::new().into(),
        metrics_gauge: HashMap::new().into(),
        metrics_histogram: HashMap::new().into(),
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
