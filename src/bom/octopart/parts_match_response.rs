use super::parts_match_request::PartsMatchRequest;
use super::parts_match_result::PartsMatchResult;

#[derive(Serialize, Deserialize, Debug)]
pub struct PartsMatchResponse {
    /// The original request.
    request: PartsMatchRequest,
    /// List of query results.
    results: Vec<PartsMatchResult>,
    /// The server response time in milliseconds.
    msec: i32,
}