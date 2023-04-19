use simetry::truck_simulator;

#[tokio::main]
async fn main() {
    let data = truck_simulator::query(truck_simulator::DEFAULT_URI)
        .await
        .unwrap();
    println!("{}", data);
}
