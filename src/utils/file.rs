use std::{fs, env};

use crate::types;
use crate::version::VERSION;


pub fn read_file_icon() {
    match fs::read_to_string(env::current_dir().unwrap().to_str().unwrap().to_owned() + "/src/static/icon.txt") {
        Ok(t) => {
            println!("{}", t);
            println!("     V.{}\n\n\n", env::var("VERSION").unwrap_or(String::from(VERSION)))
        },
        Err(e) => println!("{:?}", e),
    }
}

pub fn read_config_yaml() -> types::metrics::Metrics {
    let config_path = env::current_dir().unwrap().to_str().unwrap().to_owned() + "/expo.yaml";

    match fs::read_to_string(config_path) {
        Ok(contents) => serde_yaml::from_str::<types::metrics::Metrics>(&contents).unwrap_or_default(),
        Err(_) => types::metrics::Metrics::default(),
    }
}
