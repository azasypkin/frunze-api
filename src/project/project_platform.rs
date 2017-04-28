#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectPlatform {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub type_name: String,
    pub name: String,
    pub description: String,
    pub capabilities: Vec<String>,
}
