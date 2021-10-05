use std::prelude::*;

use crate::plantuml::code_generator;

use super::model::ClassDiagram;

pub struct PlantUMLGenerator {}

impl PlantUMLGenerator {
    pub fn new() -> PlantUMLGenerator {
        PlantUMLGenerator {}
    }

    pub fn transformSwagger2Puml(
        &self,
        spec_file: String,
        output: String,
        generate_definition_model_only: bool,
        include_cardinality: bool,
        generate_svg: bool,
    ) {
        match std::fs::try_exists(spec_file.clone()) {
            Ok(_) => {}
            Err(_) => panic!(" spec file not exsit"),
        };
        let swagger_spec = openapi::from_path(spec_file).expect("error in parsing swagger file");
        let target_file = std::fs::File::create(output).expect("create file error");

        let generator = code_generator::CodeGenerator::new(
            swagger_spec,
            target_file,
            generate_definition_model_only,
            include_cardinality,
        );
        generator.generate_puml();
        //    generator.
    }

 
}
