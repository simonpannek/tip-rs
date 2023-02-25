use tracing::error;

use bot_client::client::Client;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Create tip
    let mut tip = Client::default().await.expect("Failed to build client");

    // Start tip
    if let Err(why) = tip.start().await {
        error!("Failed to run client: {}", why);
    }
}
