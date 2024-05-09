use std::{fs, env};

use yaml_rust::YamlLoader;

use crate::types;


pub fn read_file_icon() {
    match fs::read_to_string(env::current_dir().unwrap().to_str().unwrap().to_owned() + "/src/static/icon.txt") {
        Ok(t) => {
            println!("{}", t);
            println!("     v.{}\n\n\n", env::var("VERSION").unwrap_or(String::from("1.0.0")))
        },
        Err(e) => println!("{:?}", e),
    }
}

pub fn read_config_yaml() -> types::metrics::Metrics {
    if !fs::metadata(env::current_dir().unwrap().to_str().unwrap().to_owned() + "/expo.yaml").is_ok() {
        return types::metrics::Metrics {
            ignore: vec![],
            allowed: vec![],
        };
    }

    match fs::read_to_string(env::current_dir().unwrap().to_str().unwrap().to_owned() + "/expo.yaml") {
        Ok(t) => {
            let file = YamlLoader::load_from_str(&t).unwrap();
            let f = &file[0];

            let mut ignore = vec![];
            let mut allowed = vec![];

            if !f["ignore"].is_badvalue() {
                for i in f["ignore"].as_vec().unwrap() {
                    if i.as_str() != None {
                        ignore.push(String::from(i.as_str().unwrap()));
                    }
                }
            }

            if !f["allowed"].is_badvalue() {
                for i in f["allowed"].as_vec().unwrap() {
                    if i.as_str() != None {
                        allowed.push(String::from(i.as_str().unwrap()));
                    }
                }
            }

            types::metrics::Metrics {
                ignore,
                allowed,
            }
        },
        Err(_) => {
            types::metrics::Metrics {
                ignore: vec![],
                allowed: vec![],
            }
        },
    }
}