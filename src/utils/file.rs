use std::{fs, env};

pub fn read_file_icon() {
    match fs::read_to_string(env::current_dir().unwrap().to_str().unwrap().to_owned() + "/src/static/icon.txt") {
        Ok(t) => {
            println!("{}", t)
        },
        Err(e) => println!("{:?}", e),
    }
}