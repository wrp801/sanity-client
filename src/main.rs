#[cfg(feature = "sanrs")]
use sanity_client::sanrs::entry;

#[cfg(feature = "sanrs")]
pub fn main() {
    let _ = entry::run(); // Directly use `entry::run()` since `entry` is already in scope
}

