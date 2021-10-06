use std::path::PathBuf;

use serde::*;
#[derive(Serialize, Deserialize,Debug)]
pub struct ServerOption {
    port: Option<i32>,
}
#[derive(Serialize, Deserialize,Debug)]
pub struct AppOption {
    server: ServerOption,
}

impl AppOption {
    pub fn from_path(path_string: String) {

        // let workdir=dir.to+"";
       let app_option=AppOption:: parse_app_config(path_string);
       println!("{:?}",app_option);
    }

    pub fn init() {}

    pub fn parse_app_config(path_string: String) -> std::io::Result<AppOption> {
        let mut full_path: PathBuf;
        let path = std::path::Path::new(&path_string);
        if path.is_relative() {
            let current_dir = std::env::current_dir()?;
            full_path =  current_dir.to_path_buf();
            full_path.push(path.to_path_buf());
        } else {
            full_path = path.to_path_buf();
        }
        if full_path.is_file() {
            let config_file = std::fs::read(full_path)?;
            let text = std::str::from_utf8(&config_file).expect("parse except");
            let app_config: AppOption = toml::from_str(text)?;
            Ok(app_config)
        } else {
            panic!("{:?}   is not a file", full_path);
        }
    }
}
