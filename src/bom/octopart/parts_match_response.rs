use super::parts_match_request::PartsMatchRequest;
use super::parts_match_result::PartsMatchResult;

#[derive(Serialize, Deserialize, Debug)]
pub struct PartsMatchResponse {
    /// The original request.
    pub request: PartsMatchRequest,
    /// List of query results.
    pub results: Vec<PartsMatchResult>,
    /// The server response time in milliseconds.
    pub msec: u32,
}
