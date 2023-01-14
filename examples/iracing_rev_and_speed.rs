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
        println!("Received!");
        while client.is_connected() {
            if let Some(sim_state) = client.wait_for_sim_state(Duration::from_millis(16)) {
                let rpm = f32::round(sim_state.read(&sim_state.variables()["RPM"]).unwrap_or(0.0));
                let speed = f32::round(
                    sim_state
                        .read(&sim_state.variables()["Speed"])
                        .unwrap_or(0.0)
                        * 3.6,
                );
                println!("{} km/h @ {} RPM", speed, rpm);
            }
        }
    }
}
