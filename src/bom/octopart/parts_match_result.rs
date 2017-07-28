use super::part::Part;

#[derive(Serialize, Deserialize, Debug)]
pub struct PartsMatchResult {
    /// Total number of matched items.
    pub hits: u32,
    /// Reference string specified in query.
    pub reference: Option<String>,
    /// List of matched parts.
    pub items: Vec<Part>,
    /// Error message (if applicable)
    pub error: Option<String>,
}
