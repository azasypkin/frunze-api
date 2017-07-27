#[derive(Serialize, Deserialize, Debug)]
pub struct Seller {
    /// 64-bit unique identifier.
    uid: String,
    /// The seller's display name.
    name: String,
    /// The seller's homepage url.
    homepage_url: Option<String>,
    /// ISO 3166 alpha-2 country code for display flag (eg. US).
    display_flag: Option<String>,
    /// Whether seller has e-commerce.
    has_ecommerce: Option<bool>,
}