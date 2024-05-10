mod types;
mod utils;
mod adapters;
mod application;
mod infra;
mod version;

use dotenv::dotenv;
use env_logger::Env;
use infra::app::start_app;
use utils::file::read_file_icon;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    read_file_icon();

    start_app().await
}