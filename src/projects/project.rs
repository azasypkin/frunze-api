#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    /// Identifier of the project.
    pub id: String,
    /// Name of the project.
    pub name: String,
    /// Description of the project.
    pub description: String,
    /// List of the project capabilities (from the list of supported capabilities).
    pub capabilities: Vec<String>,
    /// Project platform (from the list of supported platforms).
    pub platform: String,
}
