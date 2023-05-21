use simetry::assetto_corsa::Client;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut client = Client::connect(Duration::from_secs(1)).await;
    println!("{:#?}", client.static_data());
    while let Some(sim_state) = client.next_sim_state().await {
        println!("{:#?}", sim_state);
    }
}
