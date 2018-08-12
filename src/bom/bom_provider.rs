use std::collections::HashMap;

use super::octopart::part::Part;
use super::octopart::parts_match_query::PartsMatchQuery;
use super::octopart::parts_match_request::PartsMatchRequest;
use super::octopart::parts_match_response::PartsMatchResponse;
use actix_web::{client, HttpMessage};
use failure::Error;
use futures::Future;
use serde_json;
use url::Url;

/// Manages access to BOM related information.
#[derive(Clone)]
pub struct BomProvider {
    api_url: Url,
    api_key: String,
}

impl BomProvider {
    pub fn new<T: Into<String>>(api_url: Url, api_key: T) -> Self {
        BomProvider {
            api_url,
            api_key: api_key.into(),
        }
    }

    /// Returns Part information by its `uid`.
    pub fn get_part<T: Into<String>>(&self, uid: T) -> Result<Part, Error> {
        let url = Url::parse_with_params(
            &format!("{}/parts/{}", &self.api_url.as_str(), uid.into()),
            &[
                ("apikey", self.api_key.as_ref()),
                ("slice[offers]", "[0:1]"),
            ],
        )?;

        Ok(serde_json::from_str(&BomProvider::get(&url)?)?)
    }

    pub fn find_parts<T: Into<String>>(
        &self,
        mpns: Vec<T>,
    ) -> Result<HashMap<String, Option<Part>>, Error> {
        let request = PartsMatchRequest {
            queries: mpns
                .into_iter()
                .map(|mpn| PartsMatchQuery::new().with_mpn(mpn).with_limit(1))
                .collect(),
            exact_only: false,
        };

        let url = Url::parse_with_params(
            &format!("{}/parts/match", &self.api_url),
            &[
                ("apikey", &self.api_key),
                ("queries", &serde_json::to_string(&request.queries).unwrap()),
            ],
        )?;

        let response: PartsMatchResponse = serde_json::from_str(&BomProvider::get(&url)?)?;
        let result_map = response
            .results
            .into_iter()
            .enumerate()
            .map(|(index, mut result)| {
                let mpn_at_index = request
                    .queries
                    .get(index)
                    .and_then(|query| query.mpn.as_ref().map(|s| s.to_string()));
                (
                    mpn_at_index.unwrap_or_else(|| "unknown".to_string()),
                    result.items.drain(..).last(),
                )
            }).collect();

        Ok(result_map)
    }

    fn get(url: &Url) -> Result<String, Error> {
        client::get(url.as_str())
            .finish()
            .unwrap()
            .send()
            .map_err(|err| err.into())
            .and_then(|res| {
                res.body()
                    .from_err()
                    .and_then(|bytes| Ok(String::from_utf8(bytes.to_vec()).unwrap()))
            }).wait()
    }
}
