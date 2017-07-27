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
    pub start: i32,
    /// Maximum number of items to return. Maximum value is 20,
    /// and maximum 'start' + 'limit' is 100.
    pub limit: i32,
    /// Arbitrary string for identifying results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}
