use serde::*;
/// api  combine to  swagger api  more with custom function
#[derive(Serialize, Deserialize, Debug)]
pub struct Api {
    /// request path
    pub path: String,
    /// request method  such as  get,post,put,delete,head
    pub method: String,
    /// swagger summary
    pub summary: Option<String>,
    /// sawgger description
    pub description: Option<String>,
}
