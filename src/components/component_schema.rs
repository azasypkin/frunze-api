use std::collections::HashMap;
use components::component_property_schema::ComponentPropertySchema;

/// Describes component properties and actions.
#[derive(Serialize, Deserialize, Debug)]
pub struct ComponentSchema {
    /// Type of the component that this schema describes.
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub type_name: String,
    /// Human-readable component short name.
    pub name: String,
    /// Human-readable component long description.
    pub description: String,
    /// Property type <-> property schema map.
    pub properties: HashMap<String, ComponentPropertySchema>,
}
