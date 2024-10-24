#[cfg(feature = "sanrs")]
use {
    sanity_client::sanrs::entry
};

#[cfg(feature = "sanrs")]
#[tokio::main]
pub async fn main() {
    let _ = entry::run().await;
}

#[cfg(not(feature = "sanrs"))]
fn main() {
    println!("The sanrs feature is not enabled. Use cargo run --features sanrs for CLI use")
}
