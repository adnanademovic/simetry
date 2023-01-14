use simetry::iracing::Client;

#[tokio::main]
async fn main() {
    loop {
        println!("Starting connection to iRacing...");
        let mut client = Client::connect().await;
        println!("Connected!");
        while let Some(sim_state) = client.next_sim_state().await {
            let rpm = f32::round(sim_state.read_name("RPM").unwrap_or(0.0));
            let speed = f32::round(sim_state.read_name("Speed").unwrap_or(0.0) * 3.6);
            println!("{} km/h @ {} RPM", speed, rpm);
        }
        println!("Connection finished!");
    }
}
