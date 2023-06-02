use simetry::assetto_corsa::Client;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut client = Client::connect(Duration::from_secs(1)).await;
    while let Some(sim_state) = client.next_sim_state().await {
        println!(
            "{} km/h @ {} RPM",
            sim_state.physics.speed_kmh, sim_state.physics.rpm,
        );
    }
}
