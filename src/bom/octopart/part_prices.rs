#[derive(Serialize, Deserialize, Debug)]
pub struct PartPrices {
    #[serde(rename(deserialize = "USD"), skip_serializing_if = "Option::is_none")]
    usd: Option<Vec<(u32, String)>>,
    #[serde(rename(deserialize = "EUR"), skip_serializing_if = "Option::is_none")]
    eur: Option<Vec<(u32, String)>>,
}
