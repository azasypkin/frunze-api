use std::collections::HashMap;

/// Describes component trigger action.
#[derive(Serialize, Deserialize, Debug)]
pub struct ComponentTriggerAction {
    /// Type of the action to trigger.
    pub action: String,
    /// Identifier of the component where action of `type_name` type should be executed.
    pub component: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Component {
    /// Identifier of the project component.
    pub id: String,
    /// Type of the project component.
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub type_name: String,
    /// Values of project component properties.
    pub properties: HashMap<String, String>,
    /// Component trigger <-> corresponding actions map.
    pub triggers: HashMap<String, Vec<ComponentTriggerAction>>,
}
