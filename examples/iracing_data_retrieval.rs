use simetry::iracing::Client;
use std::time::Duration;

#[tokio::main]
async fn main() {
    loop {
        println!("Starting connection to iRacing...");
        let mut client = Client::connect(Duration::from_secs(1)).await;
        println!("Connected!");
        while let Some(sim_state) = client.next_sim_state().await {
            println!("{sim_state:#?}");
        }
        println!("Connection finished!");
    }
}
