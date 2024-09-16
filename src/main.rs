use sanity_client::SanityClient;

#[cfg(feature = "sanrs")]
use sanity_client::sanrs::entry;

#[cfg(feature = "sanrs")]
#[tokio::main]
pub async fn main() {
    let token = String::from("my-token");
    let dataset = String::from("dev");
    let project = String::from("my-project");
    let client = SanityClient::new(token, dataset, project);
    entry::run(&client).await
}
