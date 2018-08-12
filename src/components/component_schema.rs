use components::component_property_schema::ComponentPropertySchema;
use std::collections::HashMap;

/// Describes component action.
#[derive(Serialize, Deserialize, Debug)]
pub struct ComponentAction {
    /// Type of the component action.
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub type_name: String,
    /// Human-readable short action name.
    pub name: String,
    /// Human-readable action long description.
    pub description: String,
}

/// Describes component trigger.
#[derive(Serialize, Deserialize, Debug)]
pub struct ComponentTrigger {
    /// Type of the component trigger.
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub type_name: String,
    /// Human-readable short trigger name.
    pub name: String,
    /// Human-readable trigger long description.
    pub description: String,
}

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
    /// The manufacturer number of the real hardware part associated with the component if any.
    pub mpn: Option<String>,
    /// Property type <-> property schema map.
    pub properties: HashMap<String, ComponentPropertySchema>,
    /// Action type <-> action map.
    pub actions: HashMap<String, ComponentAction>,
    /// Trigger type <-> trigger map.
    pub triggers: HashMap<String, ComponentTrigger>,
}
