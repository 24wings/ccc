use serde_json::json;
pub(crate) mod model;
mod generator;
mod code_generator;
#[derive(serde::Serialize)]
enum UmlShape {
    ClassDiagram,
}

#[derive(serde::Serialize)]
enum UmlType {
    PlantUml,
    Yml,
}
#[derive(serde::Serialize)]
struct Uml {
    shape: UmlShape,
    uml_type: UmlType,
}

// #[test]
pub fn test_toml() {
    //    let  data= include_str!("swagger.json") ;
    // let spec = openapi::from_path("src/swagger.json").expect("");
    // let yaml = openapi::to_yaml(&spec).expect("");
    // std::fs::write("swagger.yaml", yaml).unwrap();
    process("src/swagger.json".to_string(), "dist".to_string(), false, true, true);
    
    // let class = model::model: { x: 2, y: 3 };
    // let uml = Uml {
    //     shape: UmlShape::ClassDiagram,
    //     uml_type: UmlType::PlantUml,
    // };

    // let toml_str = toml::to_string(&uml).unwrap_or_default();
    // println!("{}", toml_str);
}


pub fn from_openapi( spec:openapi::Spec)->String{
    "".parse().expect("")

}
/// cli command      
pub fn process(spec_file:String, output:String,generate_definition_model_only:bool,include_cardinality:bool,generate_svg:bool){
   let gen= generator::PlantUMLGenerator::new();
   gen.transformSwagger2Puml(spec_file,output,generate_definition_model_only,include_cardinality,generate_svg)

}

