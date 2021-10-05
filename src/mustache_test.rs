use serde_json::value::{self, Map, Value as Json};
use handlebars::{
    to_json, Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError,
};
static TYPES: &'static str = "serde_json";
use env_logger;
use super::hbs_test;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Team {
    name: String,
    pts: u16,
}

pub fn make_data() -> Map<String, Json> {
    let mut data = Map::new();

    data.insert("year".to_string(), to_json("2015"));

    let teams = vec![
        Team {
            name: "Jiangsu Suning".to_string(),
            pts: 43u16,
        },
        Team {
            name: "Shanghai SIPG".to_string(),
            pts: 39u16,
        },
        Team {
            name: "Hebei CFFC".to_string(),
            pts: 27u16,
        },
        Team {
            name: "Guangzhou Evergrand".to_string(),
            pts: 22u16,
        },
        Team {
            name: "Shandong Luneng".to_string(),
            pts: 12u16,
        },
        Team {
            name: "Beijing Guoan".to_string(),
            pts: 7u16,
        },
        Team {
            name: "Hangzhou Greentown".to_string(),
            pts: 7u16,
        },
        Team {
            name: "Shanghai Shenhua".to_string(),
            pts: 4u16,
        },
    ];

    data.insert("teams".to_string(), to_json(&teams));
    data.insert("engine".to_string(), to_json(TYPES));
    data
}

#[test]
fn mustache_test(){
    env_logger::init();
    let  dir=  std::env::current_dir().expect("");
    let mut current_dir=String::new();
    match dir.to_str() {
      Some(str)=>{
          println!("{}",str);
          current_dir=str.to_string();
  
        },
        None=>println!("none")
    }
   println!("{}",current_dir);
  let r=   mustache::compile_path(current_dir+"/resource/handlebars/java/java.mustache");
  match r{

      Ok(template)=>{
         let  data= hbs_test::make_data();
        // let mut data=    HashMap::new();
        let mut bytes=vec![];
        // data.insert("ok",  true);
       template.render(&mut bytes, &data).expect("errnor template");
       println!("template:{}",std::str::from_utf8(&bytes).expect("error bytes"));
      },
      Err(err)=>println!("err:{}",err)
  }
//   println!("{}",r)   

}