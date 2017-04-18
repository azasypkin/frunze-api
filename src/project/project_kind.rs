#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectKind {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub type_name: String,
    pub name: String,
}
