use simetry::iracing::Client;
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("Starting connection to iRacing...");
    let mut client = Client::connect().await;
    println!("Connected to memory interface!");
    loop {
        while !client.wait_for_data(Duration::from_millis(1000)) {
            println!("Waiting for iRacing data...");
        }
        println!("Received!");
        while client.is_connected() {
            if !client.wait_for_data(Duration::from_millis(16)) {
                continue;
            }
            let data = match client.data() {
                None => continue,
                Some(v) => v,
            };
            let rpm = f32::round(data.read(&data.vars["RPM"]).unwrap_or(0.0));
            let speed = f32::round(data.read(&data.vars["Speed"]).unwrap_or(0.0) * 3.6);
            println!("{} km/h @ {} RPM", speed, rpm);
        }
    }
}
