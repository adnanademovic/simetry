use simetry::iracing::Client;
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("Starting connection to iRacing...");
    let mut client = Client::connect().await;
    println!("Connected to memory interface!");
    loop {
        while client
            .wait_for_sim_state(Duration::from_millis(1000))
            .is_none()
        {
            println!("Waiting for iRacing data...");
        }
        while client.is_connected() {
            if let Some(sim_state) = client.wait_for_sim_state(Duration::from_millis(16)) {
                println!("{sim_state:#?}");
            }
        }
    }
}
