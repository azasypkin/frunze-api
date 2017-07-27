use super::part_prices::PartPrices;
use super::seller::Seller;

#[derive(Serialize, Deserialize, Debug)]
pub struct PartOffer {
    /// The seller's part number.
    sku: String,
    /// Object representing the seller.
    seller: Seller,
    /// The (ISO 3166-1 alpha-2) or (ISO 3166-2) code indicating the geo-political region(s) for
    /// which offer is valid (eg. US-NY).
    eligible_region: Option<String>,
    /// URL for seller landing page.
    product_url: Option<String>,
    /// URL for generating RFQ through Octopart.
    octopart_rfq_url: Option<String>,
    /// Dictionary mapping currencies to lists of (Break, Price) tuples.
    prices: PartPrices,
    /// Number of parts seller has available.
    in_stock_quantity: i32,
    /// Number of parts on order from factory.
    on_order_quantity: Option<i32>,
    /// ISO 8601 formatted ETA of order from factory.
    on_order_eta: Option<String>,
    /// Number of days to acquire parts from factory.
    factory_lead_days: Option<i32>,
    /// Order multiple for factory orders.
    factory_order_multiple: Option<i32>,
    /// Number of items which must be ordered together
    order_multiple: Option<i32>,
    /// Minimum order quantity.
    moq: Option<i32>,
    /// Form of offer packaging (e.g. reel, tape).
    /// TODO Use Enum.
    packaging: Option<String>,
    /// True if seller is authorized by manufacturer.
    is_authorized: bool,
    /// ISO 8601 formatted time when offer was last updated by the seller.
    last_updated: String,
}
