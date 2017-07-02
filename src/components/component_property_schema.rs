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
    pub default: String,
}
