use simetry::raceroom_racing_experience::Client;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut client = Client::connect(Duration::from_secs(1)).await;
    println!("{:#?}", client.next_sim_state().await);
}
