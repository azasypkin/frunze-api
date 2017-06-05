use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectComponent {
    /// Identifier of the project component.
    pub id: String,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub type_name: String,
    pub name: String,
    pub description: String,
    pub properties: HashMap<String, String>,
}
