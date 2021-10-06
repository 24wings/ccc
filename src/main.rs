// #![allow(unused_imports, dead_code)]
#![feature(path_try_exists)]

extern crate env_logger;
extern crate handlebars;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
use openapi::Spec;
use serde_json::Value;
// use clap::{App, Arg,SubCommand};
#[macro_use] extern crate rocket;

// use jsonschema_valid;

mod test;
// mod hbs_test;
// mod mustache_test;
// mod plantuml;
mod app_option;
extern crate clap;

// extern crate openapi;






#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {

   let option= app_option::AppOption::from_path("./examples/simple-config/config.toml".to_string());


    
    rocket::build().mount("/", routes![index])
}