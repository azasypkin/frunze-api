use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Component {
    /// Identifier of the project component.
    pub id: String,
    /// Type of the project component.
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub type_name: String,
    /// Values of project component properties.
    pub properties: HashMap<String, String>,
}
