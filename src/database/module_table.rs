#[crud_table]
#[derive(Clone, Debug)]
pub struct ModuleTable{
    id:i32,
    module_json:Option<String>

}