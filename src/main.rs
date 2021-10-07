// #![allow(unused_imports, dead_code)]
// #![feature(path_try_exists)]
#[macro_use]
extern crate lazy_static;

extern crate env_logger;
extern crate handlebars;
extern crate rbatis;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
use figment::Figment;

// use clap::{App, Arg,SubCommand};
#[macro_use]
extern crate rocket;

// use jsonschema_valid;

mod test;
// mod hbs_test;
// mod mustache_test;
// mod plantuml;
mod app_option;
extern crate clap;
mod database;
mod dynamic;
mod utils;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    database::connect_to_db().await;
    let option = app_option::AppOption::init("./examples/simple-config/config.toml".to_string());

    println!("{:#?}", option);
    let mut figment = Figment::from(rocket::Config::default());
    figment = figment.merge(("port", option.server.port));
    figment = figment.merge(("address", option.server.host));

    rocket::custom(figment).mount(
        "/",
        routes![
            index,
            dynamic::dynamic_module_controller::dynamic_controller
        ],
    )
}
