use simetry::iracing::Client;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut client = Client::connect(Duration::from_secs(1)).await;
    let sim_state = client.next_sim_state().await.unwrap();
    let variables = sim_state.variables();
    println!("{variables:#?}");
}
