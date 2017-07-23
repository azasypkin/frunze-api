/// Describes component property predefined value.
#[derive(Serialize, Deserialize, Debug)]
pub struct ComponentPropertyPredefinedValue {
    /// Property value option type.
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub type_name: String,
    /// Human-readable short value option type name.
    pub name: String,
    /// Human-readable value option type long description.
    pub description: String,
}

/// Describes kind of component property.
#[derive(Serialize, Deserialize, Debug)]
pub enum ComponentPropertyValueKind {
    /// Value is arbitrary string.
    #[serde(rename(serialize = "custom", deserialize = "custom"))]
    Custom,
    /// Value is string limited by the predefined list of options.
    #[serde(rename(serialize = "predefined", deserialize = "predefined"))]
    Predefined(Vec<ComponentPropertyPredefinedValue>),
    /// Value is identifier of the component which type is limited by the predefined list of
    /// component types.
    #[serde(rename(serialize = "component", deserialize = "component"))]
    Component(Vec<String>),
}

/// Describes single component property.
#[derive(Serialize, Deserialize, Debug)]
pub struct ComponentPropertySchema {
    /// Property type.
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub type_name: String,
    /// Human-readable short property name.
    pub name: String,
    /// Human-readable property long description.
    pub description: String,
    /// Default value of the property.
    #[serde(rename(serialize = "defaultValue", deserialize = "defaultValue"))]
    pub default_value: String,
    /// Property value kind.
    pub kind: ComponentPropertyValueKind,
}