use simetry::truck_simulator;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let data = truck_simulator::TruckSimulatorClient::connect(
        truck_simulator::DEFAULT_URI,
        Duration::from_secs(1),
    )
    .await
    .query()
    .await
    .unwrap();
    println!("{:#?}", data);
}
