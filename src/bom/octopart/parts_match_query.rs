/// See https://octopart.com/api/docs/v3/rest-api#response-schemas-partsmatchquery.
#[derive(Serialize, Deserialize, Debug)]
pub struct PartsMatchQuery {
    /// Free-form keyword query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    /// MPN search filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpn: Option<String>,
    /// Brand search filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    /// SKU search filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    /// Seller search filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller: Option<String>,
    /// MPN or SKU search filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpn_or_sku: Option<String>,
    /// Ordinal position of first returned item.
    pub start: u32,
    /// Maximum number of items to return. Maximum value is 20,
    /// and maximum 'start' + 'limit' is 100.
    pub limit: u32,
    /// Arbitrary string for identifying results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

impl PartsMatchQuery {
    /// Returns `PartsMatchQuery` with only `mpn` field filled.
    pub fn by_mpn<T: Into<String>>(mpn: T) -> PartsMatchQuery {
        PartsMatchQuery {
            q: None,
            mpn: Some(mpn.into()),
            brand: None,
            sku: None,
            seller: None,
            mpn_or_sku: None,
            start: 0,
            limit: 10,
            reference: None,
        }
    }

    /// Consumes itself to set `limit` field.
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = limit;
        self
    }
}
