
#[derive(Serialize,Deserialize)]

pub struct ClassDiagram {
  pub class_name: String,
  pub is_class: bool,
  pub description: String,

  pub super_class: String,
  pub fields: Vec<ClassMembers>,
  pub child_class: Vec<ClassRelation>,
}
impl ClassDiagram {
  /// create new class
  pub fn new(
    class_name: String,
    is_class: bool,
    description: String,
    super_class: String,
    fields: Vec<ClassMembers>,
    child_class: Vec<ClassRelation>,
  ) -> ClassDiagram {
    ClassDiagram {
      class_name,
      is_class,
      description,
      super_class,
      fields,
      child_class,
    }
  }
}

#[derive(Serialize,Deserialize)]
pub struct ClassMembers {
  data_type: String,
  name: String,
  class_name: String,
  ///
  cardinality: String,
}

#[derive(Serialize,Deserialize)]

pub struct ClassRelation {
  target: String,
  is_extension: bool,
  is_composition: bool,
  cardinality: String,
  source_class: String,
}

#[derive(Serialize,Deserialize)]

pub struct InterfaceDiagram {
  interface_name: String,
  methods: Vec<MethodDefinitions>,
  child_class: Vec<ClassRelation>,
  error_class: String,
}

#[derive(Serialize,Deserialize)]

pub struct MethodDefinitions {
  method_definition:String,
  return_type:String
}

