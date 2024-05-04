use std::{fs, env};

pub fn read_file_icon() {
    match fs::read_to_string(env::current_dir().unwrap().to_str().unwrap().to_owned() + "/src/static/icon.txt") {
        Ok(t) => {
            println!("{}", t);
            println!("     v.{}\n\n\n", env::var("VERSION").unwrap_or(String::from("1.0.0")))
        },
        Err(e) => println!("{:?}", e),
    }
}