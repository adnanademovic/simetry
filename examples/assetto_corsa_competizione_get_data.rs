use simetry::assetto_corsa_competizione::SharedMemoryClient;

#[tokio::main]
async fn main() {
    let client = SharedMemoryClient::connect().await.unwrap();
    println!("{:#?}", client.static_data());
    println!("{:#?}", client.graphics());
    println!("{:#?}", client.physics());
}
