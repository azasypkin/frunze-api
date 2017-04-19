use project::project_capability::ProjectCapability;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectCapabilityGroup {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub type_name: String,
    pub name: String,
    pub hint: String,
    pub capabilities: Vec<ProjectCapability>,
}
