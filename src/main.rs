use nostr_sdk::prelude::*;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    // Create new client
    let my_keys: Keys = Keys::generate();
    let client = Client::new(&my_keys);

    // Connect to a relay
    client.add_relay("wss://relay.damus.io", None).await?;
    client.connect().await;

    // Get event
    let id = String::from("4aac0ffb020ed44e4f54a39ac1897f9bc6512d0e0fb5e29226d14a88b1c58a9c");
    let timeout = Some(Duration::from_secs(60));
    let filter = Filter::new().ids(vec![id]);
    let events = client.get_events_of(vec![filter], timeout).await?;
    println!("{events:?}");

    // Disconnect
    client.disconnect().await?;

    Ok(())
}
