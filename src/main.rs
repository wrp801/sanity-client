#[cfg(feature = "sanrs")]
use {
    sanity_client::SanityClient,
    sanity_client::sanrs::{config, entry}
};

#[cfg(feature = "sanrs")]
#[tokio::main]
pub async fn main() {
    let _ = entry::run().await;
}
