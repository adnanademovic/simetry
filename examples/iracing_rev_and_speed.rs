use simetry::iracing::Client;

#[tokio::main]
async fn main() {
    println!("Starting connection to iRacing...");
    let mut client = Client::initialize().await;
    println!("Connected to memory interface!");
    loop {
        println!("Connecting to new client...");
        for sim_state in client.run_connection() {
            let rpm = f32::round(sim_state.read_name("RPM").unwrap_or(0.0));
            let speed = f32::round(sim_state.read_name("Speed").unwrap_or(0.0) * 3.6);
            println!("{} km/h @ {} RPM", speed, rpm);
        }
        println!("Connection finished!");
    }
}
