#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectCapability {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub type_name: String,
    pub name: String,
    pub kinds: Vec<String>,
}
