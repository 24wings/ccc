use crate::app_option::module::Module;
// use crate::dynamic::APPLICATIONCONTENT;

mod api;
mod controller;
mod dto;
mod module;
use super::utils;
use serde::*;
use serde_json::*;
#[derive(Serialize, Deserialize, Debug)]
pub struct ServerOption {
    #[serde(default = "default_port")]
    pub port: i32,
    #[serde(default = "default_host")]
    pub host: String,
    pub log_level: Option<String>,
    pub dir: Option<String>,
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}
fn default_port() -> i32 {
    80
}

/// swagger base option
#[derive(Serialize, Deserialize, Debug)]
pub struct SwaggerBaseOption {
    pub info: SwaggerInfo,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SwaggerInfo {
    title: String,
    version: String,
    #[serde(rename = "termsOfService")]
    terms_of_service: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppOption {
    pub server: ServerOption,
    pub swagger: SwaggerBaseOption,
}

/// application all  modules  every module contains controller, dto,entity,
///
#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationContent {
    pub modules: Vec<Module>,
}
impl ApplicationContent {
    pub fn set_modules(mut self, modules: Vec<Module>) {
        self.modules = modules;
    }
}

// impl DerefMut for ApplicationContent {
//     fn deref_mut(&mut self) -> &mut ApplicationContent {
//         self
//     }
// }

impl AppOption {
    // pub fn from_path(path_string: String) -> std::io::Result<AppOption> {
    //     AppOption::parse_app_config(path_string)
    // }

    pub fn init(path_string: String) -> AppOption {
        log::info!("loading modules start");
        let option = AppOption::parse_app_config(path_string).unwrap();
        let modules = Module::modules_from_dir(option.server.dir.clone().unwrap());
        let application_content = ApplicationContent { modules };
        // log::info!("{:?}", modules);
        let str = json!({"option":option,"application_content":application_content});
        std::fs::write("db.json", str.to_string());
        option
    }

    pub fn parse_app_config(path_string: String) -> std::io::Result<AppOption> {
        let full_path = utils::path_util::get_absolute_path_from_current_dir(path_string).unwrap();
        if full_path.is_file() {
            let config_file = std::fs::read(full_path)?;
            let text = std::str::from_utf8(&config_file).expect("parse except");
            let app_config: AppOption = toml::from_str(text)?;
            Ok(app_config)
        } else {
            panic!("{:#?}   is not a file", full_path);
        }
    }
}
