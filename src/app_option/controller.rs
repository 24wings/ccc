use crate::app_option::api::Api;
use serde::*;
#[derive(Serialize, Deserialize, Debug)]
pub struct Controller {
    pub base_path: String,
    pub tags: Vec<String>,
    pub apis: Vec<Api>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ControllerFile {
    controller: Vec<Controller>,
}

impl Controller {
    pub fn from_file(file_path: String) -> Vec<Controller> {
        let file_abs_path =
            super::super::utils::path_util::get_absolute_path_from_current_dir(file_path).unwrap();
        if file_abs_path.is_file() {
            let bytes: Vec<u8> = std::fs::read(file_abs_path).unwrap();
            let content: &str = std::str::from_utf8(&bytes).unwrap();
            let controller_file: ControllerFile = toml::from_str(content).unwrap();
            controller_file.controller
        } else {
            panic!("{:?} is not a file", file_abs_path);
        }
    }
}
