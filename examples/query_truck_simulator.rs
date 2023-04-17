use simetry::truck_simulator;

#[tokio::main]
async fn main() {
    let data = truck_simulator::query().await.unwrap();
    println!("{}", data);
}
