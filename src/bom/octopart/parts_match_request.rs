use super::parts_match_query::PartsMatchQuery;

#[derive(Serialize, Deserialize, Debug)]
pub struct PartsMatchRequest  {
    /// List of PartsMatchQuery objects. The maximum number of queries per request is 20.
    pub queries: Vec<PartsMatchQuery>,
    /// Match on non-alphanumeric characters in part numbers.
    pub exact_only: bool,
}