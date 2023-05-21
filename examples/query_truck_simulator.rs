use simetry::truck_simulator;

#[tokio::main]
async fn main() {
    let data = truck_simulator::TruckSimulatorClient::connect(truck_simulator::DEFAULT_URI)
        .await
        .unwrap()
        .query()
        .await
        .unwrap();
    println!("{:#?}", data);
}
