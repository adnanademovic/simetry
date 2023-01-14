use simetry::iracing::Client;

#[tokio::main]
async fn main() {
    println!("Starting connection to iRacing...");
    let mut client = Client::initialize().await;
    println!("Connected to memory interface!");
    loop {
        println!("Connecting to new client...");
        for sim_state in client.run_connection() {
            println!("{sim_state:#?}");
        }
        println!("Connection finished!");
    }
}
