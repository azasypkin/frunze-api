use super::part_offer::PartOffer;

#[derive(Serialize, Deserialize, Debug)]
pub struct Part {
    /// 64-bit unique identifier.
    uid: String,
    /// The manufacturer part number.
    mpn: String,
    /// The url of the Octopart part detail page.
    octopart_url: String,
    /// List of offer objects.
    offers: Vec<PartOffer>,
}