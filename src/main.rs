use sanity_client::SanityClient;

#[cfg(feature = "sanrs")]
use sanity_client::sanrs::entry;

#[cfg(feature = "sanrs")]
#[tokio::main]
pub async fn main() {
    let token = String::from("skmeoSwqPMxAk4knoltd4QYdOEBt1kcbpVAuNK1rjHMFASEr98v09LwpgcP4SUzIbiilHOUW2nGauqU4mZlL75hFFIXLLFkP3Or5VNOj7oZBGeoWifPRW2JWCI0S076yRFaiydChbod79f0U5juBciZkdYGw03ZWu10MmaAfLOf0aubASPzo");
    let dataset = String::from("dev");
    let project = String::from("2xyydva6");
    let client = SanityClient::new(token, dataset, project);
    entry::run(&client).await
}
