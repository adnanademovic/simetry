use simetry::assetto_corsa_competizione::SharedMemoryClient;

#[tokio::main]
async fn main() {
    let mut client = SharedMemoryClient::connect().await.unwrap();
    while let Some(sim_state) = client.next_sim_state().await {
        println!(
            "{} km/h @ {} RPM",
            sim_state.physics.speed_kmh, sim_state.physics.rpm,
        );
    }
}
