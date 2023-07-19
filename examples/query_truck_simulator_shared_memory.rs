use simetry::truck_simulator;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let data = truck_simulator::Client::connect(Duration::from_secs(1))
        .await
        .next_sim_state()
        .await
        .unwrap();
    println!("{:#?}", data);
}
