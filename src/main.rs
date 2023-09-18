mod types;
mod routes;
mod utils;

use std::collections::HashMap;

use actix_web::{App, HttpServer, web, middleware::Logger};
use dotenv::dotenv;
use env_logger::Env;
use prometheus::Registry;
use routes::{druid::druid_handler, metrics::metrics_handler, health::health_handler};
use types::app_state::AppState;
use utils::file::read_file_icon;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    read_file_icon();

    let data = web::Data::new(AppState {
        registry: Registry::new().into(),
        metrics_gauge: HashMap::new().into(),
        metrics_histogram: HashMap::new().into(),
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