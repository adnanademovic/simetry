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
        if let Some(data) = client.get_data() {
            for (key, val) in data.vars {
                println!("{}: {:?}", key, val);
            }
        }
        if let Ok(session_info) = client.get_session_info_string() {
            println!("Session info: {}", session_info);
        }
        println!("Received!");
        while client.is_connected() {
            if client.wait_for_data(Duration::from_millis(16)) {
                println!("Received new data");
            } else {
                println!("Waiting for further data");
            }
        }
    }
}
