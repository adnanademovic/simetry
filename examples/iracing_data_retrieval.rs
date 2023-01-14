use simetry::iracing::Client;

#[tokio::main]
async fn main() {
    loop {
        println!("Starting connection to iRacing...");
        let mut client = Client::connect().await.unwrap();
        println!("Connected!");
        while let Some(sim_state) = client.next_sim_state().await {
            println!("{sim_state:#?}");
        }
        println!("Connection finished!");
    }
}
