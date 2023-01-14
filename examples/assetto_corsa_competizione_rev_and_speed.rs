use simetry::assetto_corsa_competizione::SharedMemoryClient;

#[tokio::main]
async fn main() {
    let client = SharedMemoryClient::connect().await.unwrap();
    while client.is_connected() {
        let physics = client.physics();
        println!("{} km/h @ {} RPM", physics.speed_kmh, physics.rpm);
    }
}
