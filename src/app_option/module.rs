use super::super::utils::path_util;
use super::controller::Controller;
use serde::*;
use std::fs::ReadDir;
use std::path::PathBuf;
use std::str::FromStr;
#[derive(Serialize, Deserialize, Debug)]
pub struct Module {
    pub controllers: Vec<Controller>,
}

impl Module {
    /// from dir to create module ,
    /// module directory shoudle  be like
    ///  ```yaml
    /// - module
    ///     -passport
    ///     -rbac
    ///     -staff
    ///     -customer
    ///         - controller.toml
    ///         - dto.toml
    ///         - entity.toml
    ///         
    /// ```
    #[warn(unused_assignments)]

    pub fn module_from_dir(dir: String) -> Module {
        let full_path: PathBuf = path_util::get_absolute_path_from_current_dir(dir).unwrap();
        if full_path.is_dir() {
            log::info!("found module  dir{}", full_path.to_str().unwrap());
            let mut controler_path: PathBuf = full_path.clone();
            controler_path.push(PathBuf::from_str("controllers.toml").unwrap());
            let mut controllers = vec![];
            if controler_path.is_file() {
                log::info!("found controller file: {:#?}", controler_path);
                controllers = super::controller::Controller::from_file(
                    controler_path.to_str().unwrap().to_string(),
                );
            } else {
                panic!("{:?} is not a controllers toml file", controler_path);
            }
            Module { controllers }
        } else {
            panic!("{:?} is not a module dir", full_path);
        }
    }

    pub fn modules_from_dir(dir: String) -> Vec<Module> {
        let full_path: PathBuf = path_util::get_absolute_path_from_current_dir(dir).unwrap();
        if full_path.is_dir() {
            log::info!("found root module  dir{}", full_path.to_str().unwrap());
            let mut modules = vec![];
            let dirs: ReadDir = full_path.read_dir().unwrap();
            for dir in dirs {
                if dir.as_ref().unwrap().path().is_dir() {
                    let module_dir = dir.unwrap().path().to_str().clone().unwrap().to_string();
                    log::info!("module  dir:{:#?}", module_dir);
                    let module: Module = Module::module_from_dir(module_dir);
                    log::info!("module :{:#?}", module);

                    modules.push(module);
                }
            }
            return modules;
        } else {
            panic!("{:?} is a file ,not a modules dir", full_path);
        }
    }
}
