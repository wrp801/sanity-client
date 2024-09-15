use sanity_client::SanityClient;

#[cfg(feature = "sanrs")]
use sanity_client::sanrs::entry;

#[cfg(feature = "sanrs")]
pub fn main() {
    let _ = entry::run();
}
