use simetry::iracing::Client;
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("Starting connection to iRacing...");
    let mut client = Client::connect().await;
    println!("Connected to memory interface!");
    loop {
        let sim_state;
        loop {
            if let Some(s) = client.wait_for_sim_state(Duration::from_millis(1000)) {
                sim_state = s;
                break;
            } else {
                println!("Waiting for iRacing data...");
            }
        }
        for (key, val) in sim_state.variables() {
            println!("{}: {:?}", key, val);
        }
        if let Ok(session_info) = sim_state.get_session_info_string() {
            println!("Session info: {}", session_info);
        }
        println!("Received!");
        while client.is_connected() {
            if client
                .wait_for_sim_state(Duration::from_millis(16))
                .is_some()
            {
                println!("Received new data");
            } else {
                println!("Waiting for further data");
            }
        }
    }
}
