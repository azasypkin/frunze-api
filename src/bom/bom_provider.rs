use errors::Result;
use futures::{future, Future, Stream};
use hyper::{Error as HyperError, Client};
use tokio_core::reactor::Core;
use serde_json;
use url::Url;

use super::octopart::part::Part;
use super::octopart::parts_match_request::PartsMatchRequest;
use super::octopart::parts_match_query::PartsMatchQuery;
use super::octopart::parts_match_response::PartsMatchResponse;

/// Manages access to BOM related information.
#[derive(Clone)]
pub struct BomProvider {
    api_url: String,
    api_key: String,
}

impl BomProvider {
    pub fn new<T: Into<String>>(api_url: T, api_key: T) -> Self {
        BomProvider {
            api_url: api_url.into(),
            api_key: api_key.into(),
        }
    }

    /// Returns Part information by it's `uid`.
    pub fn get_part<T: Into<String>>(&self, uid: T) -> Result<Part> {
        let mut core = Core::new()?;
        let client = Client::new(&core.handle());

        let url = Url::parse_with_params(
            &format!("{}/parts/{}", &self.api_url, uid.into()),
            &[
                ("apikey", self.api_key.as_ref()),
                ("slice[offers]", "[0:1]"),
            ],
        )?;

        let work = client.get(url.into_string().parse()?).and_then(|res| {
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

        Ok(serde_json::from_str(&core.run(work)?)?)
    }

    pub fn find_parts<T: Into<String>>(&self, mpn: T) -> Result<Vec<Part>> {
        let mut core = Core::new()?;
        let client = Client::new(&core.handle());

        let request = PartsMatchRequest {
            queries: vec![PartsMatchQuery::by_mpn(mpn).with_limit(1)],
            exact_only: false,
        };

        let url = Url::parse_with_params(
            &format!("{}/parts/match", &self.api_url),
            &[
                ("apikey", &self.api_key),
                ("queries", &serde_json::to_string(&request.queries).unwrap()),
            ],
        )?;

        let work = client.get(url.into_string().parse()?).and_then(|res| {
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

        let mut response: PartsMatchResponse = serde_json::from_str(&core.run(work)?)?;
        if response.results.is_empty() {
            return Ok(vec![]);
        }

        let mut first_result = response.results.swap_remove(0);
        let parts = first_result.items.drain(..).collect();
        Ok(parts)
    }
}
