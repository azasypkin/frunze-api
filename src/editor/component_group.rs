use editor::component_metadata::ComponentMetadata;

#[derive(Serialize, Deserialize, Debug)]
pub struct ComponentGroup {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub type_name: String,
    pub name: String,
    pub description: String,
    pub items: Vec<ComponentMetadata>,
}
