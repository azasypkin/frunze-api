#[derive(Serialize, Deserialize, Debug)]
pub struct ComponentMetadata {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub type_name: String,
    pub name: String,
    pub description: String,
}
