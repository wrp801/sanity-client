#[cfg(feature = "sanrs")]
use {
    sanity_client::sanrs::entry
};

#[cfg(feature = "sanrs")]
#[tokio::main]
pub async fn main() {
    let _ = entry::run().await;
}
