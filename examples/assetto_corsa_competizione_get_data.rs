use simetry::assetto_corsa_competizione::SharedMemoryClient;

#[tokio::main]
async fn main() {
    let mut client = SharedMemoryClient::connect().await.unwrap();
    println!("{:#?}", client.static_data());
    while let Some(sim_state) = client.next_sim_state().await {
        println!("{:#?}", sim_state);
    }
}
