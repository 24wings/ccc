use serde::*;
/// dto model
#[derive(Serialize, Deserialize, Debug)]
pub struct Dto {
    /// dto type
    pub r#type: String,
}
