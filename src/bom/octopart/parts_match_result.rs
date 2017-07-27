use super::part::Part;

#[derive(Serialize, Deserialize, Debug)]
pub struct PartsMatchResult {
    /// Total number of matched items.
    hits: i32,
    /// Reference string specified in query.
    reference: Option<String>,
    /// List of matched parts.
    items: Vec<Part>,
    /// Error message (if applicable)
    error: Option<String>,
}
