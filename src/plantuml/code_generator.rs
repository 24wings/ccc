use openapi::{Schema, Spec};
use serde_json::{json, to_string};
use std::prelude::*;
use std::{
    collections::BTreeMap,
    env::{self, current_dir},
    fs::File,
    path::PathBuf,
};

use crate::plantuml::model::ClassDiagram;

use super::model::ClassMembers;

pub struct CodeGenerator {
    generate_definition_model_only: bool,
    include_cardinality: bool,
    swagger: Spec,
    target_location: File,
}
enum CodeGeneratorProp {
    String(String),
}

impl CodeGenerator {
    ///
    pub fn generate_puml(&self) -> String {
        println!("generate plantuml start");
        let addtion_propertys = self.process_swagger();
        let current_dir: String = env!("CARGO_MANIFEST_DIR").to_string();
        println!("{}", current_dir);
        let r =
            mustache::compile_path(current_dir + "/resource/handlebars/swagger2puml/puml.mustache");
        let t = match r {
            Ok(template) => {
                let mut result: Vec<u8> = vec![];
                template
                    .render(&mut result, &addtion_propertys)
                    .expect("error in render");
                std::str::from_utf8(&mut result)
                    .expect("error in p")
                    .to_string()
            }
            Err(_) => "".to_string(),
        };
        println!("result:{}", t);
        t
    }
    /// create a plantuml code generator
    pub fn new(
        swagger: openapi::Spec,
        target_location: File,
        generate_definition_model_only: bool,
        include_cardinality: bool,
    ) -> CodeGenerator {
        CodeGenerator {
            swagger,
            include_cardinality,
            target_location,
            generate_definition_model_only,
        }
    }
    ///extract information from
    pub fn process_swagger(&self) -> BTreeMap<String, serde_json::Value> {
        let mut map = BTreeMap::new();
        map.insert("title".to_string(), json!(self.swagger.info.title));
        map.insert("version".to_string(), json!(self.swagger.info.version));
        let class_diagrams = self.process_swagger_models();
        map.insert("classDiagrams".to_string(), json!(class_diagrams));

        return map;
    }

    fn process_swagger_models(&self) -> Vec<ClassDiagram> {
        let mut result: Vec<ClassDiagram> = vec![];

        for (key, value) in self.swagger.definitions.iter() {
            let super_class = self.get_super_class(&value);
            result.push(ClassDiagram {
                class_name: key.to_string(),
                is_class: self.is_model_class(value),
                description: value.description.as_ref().unwrap_or(&String::new()).to_string(),
                super_class,
                fields: vec![],
                child_class: vec![],
            })
        }

        result
    }

    fn get_class_members(
        &self,
        schema: Schema,
        models_map: BTreeMap<String, Schema>,
    ) -> Vec<ClassMembers> {
        let result: Vec<ClassMembers> = vec![];

        result
    }

    fn get_super_class(&self, model: &Schema) -> String {
        let mut super_class = String::new();
        let schema_type = model
            .schema_type
            .as_ref()
            .expect("error on schema type")
            .to_string();
        println!("{}", schema_type);

        if schema_type == "array" {
            let property_object = model.items.as_ref().unwrap();
            if property_object.ref_path.is_none() == false {
                let simple_ref = self.compute_simple_ref(property_object.ref_path.clone().unwrap());
                super_class = format!("{}{}]", "ArrayList[", simple_ref);
            }
        } else if schema_type == "object" {
            // model.properties
            //:todo  there is need  schema property: addtionProperties
        }

        super_class
    }
    ///  get from  $ref path to simple  ref
    pub fn compute_simple_ref(&self, ref_s: String) -> String {
        match ref_s.find("/") {
            Some(index) => ref_s[index..].to_string(),
            None => ref_s,
        }
    }

    pub fn is_model_class(&self, model: &Schema) -> bool {
        let mut is_model_class = true;
        match  model.ref_path.as_ref(){
            Some(_)=>(),
            None=>return is_model_class
        }
        if model.ref_path.as_ref().unwrap() == "object" {
            match model.enum_values.as_ref() {
                Some(v) => {
                    if v.len() > 0 {
                        is_model_class = false;
                    }
                }
                None => (),
            }
        }
        is_model_class
    }
}
