use core::control_metadata::ControlMetadata;

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlGroup {
    #[serde(rename(serialize = "type"))]
    pub type_name: String,
    pub name: String,
    pub description: String,
    pub items: Vec<ControlMetadata>,
}
