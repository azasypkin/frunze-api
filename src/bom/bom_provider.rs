use std::collections::HashMap;

use futures::{future, Future, Stream};
use hyper::{Client, Error as HyperError};
use serde_json;
use tokio_core::reactor::Core;
use url::Url;

use errors::{Error, Result};
use super::octopart::part::Part;
use super::octopart::parts_match_request::PartsMatchRequest;
use super::octopart::parts_match_query::PartsMatchQuery;
use super::octopart::parts_match_response::PartsMatchResponse;

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

    /// Returns Part information by it's `uid`.
    pub fn get_part<T: Into<String>>(&self, uid: T) -> Result<Part> {
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
    ) -> Result<HashMap<String, Option<Part>>> {
        let request = PartsMatchRequest {
            queries: mpns.into_iter()
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
            })
            .collect();

        Ok(result_map)
    }

    fn get(url: &Url) -> Result<String> {
        let mut core = Core::new()?;
        let client = Client::new(&core.handle());

        let work = client.get(url.as_str().parse()?).and_then(|res| {
            res.body()
                .fold(Vec::new(), |mut v: Vec<u8>, chunk| {
                    v.extend(&chunk[..]);
                    future::ok::<_, HyperError>(v)
                })
                .and_then(|chunks| {
                    let s = String::from_utf8(chunks).unwrap();
                    future::ok::<_, HyperError>(s)
                })
        });

        core.run(work).map_err(|e| -> Error { e.into() })
    }
}
