#[get("/api/modules/<module>/<controller>/<path>")]
pub fn dynamic_controller(module: &str, controller: &str, path: &str) -> String {
    format!("module:{},controller:{},path:{}", module, controller, path)
}
