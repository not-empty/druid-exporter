mod types;
mod routes;
mod utils;

use std::collections::HashMap;

use actix_web::{App, HttpServer, web, middleware::Logger};
use env_logger::Env;
use prometheus::Registry;
use routes::{druid::druid_handler, metrics::metrics_handler};
use types::app_state::AppState;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let data = web::Data::new(AppState {
        registry: Registry::new().into(),
        metrics_gauge: HashMap::new().into(),
        metrics_histogram: HashMap::new().into(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::PayloadConfig::new(20971520))
            .app_data(data.clone())
            .service(druid_handler)
            .service(metrics_handler)
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 7080))?
    .run()
    .await
}