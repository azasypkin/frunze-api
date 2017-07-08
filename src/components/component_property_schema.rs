/// Describes component property value option.
#[derive(Serialize, Deserialize, Debug)]
pub struct ComponentPropertyValueOption {
    /// Property value option type.
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub type_name: String,
    /// Human-readable short value option type name.
    pub name: String,
    /// Human-readable value option type long description.
    pub description: String,
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
    /// Property value kind (string or options).
    pub kind: String,
    /// Possible value options (optional, available only for properties with `kind == options`.
    pub options: Option<Vec<ComponentPropertyValueOption>>,
}
