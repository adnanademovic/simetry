use simetry::truck_simulator;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let data = truck_simulator::json_client::Client::connect(
        truck_simulator::json_client::DEFAULT_URI,
        Duration::from_secs(1),
    )
    .await
    .query()
    .await
    .unwrap();
    println!("{:#?}", data);
}
