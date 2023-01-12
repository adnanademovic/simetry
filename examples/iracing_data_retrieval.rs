use simetry::iracing_basic_solution::Client;
use std::time::Duration;

fn main() {
    let mut client = Client::new();
    loop {
        while !client.wait_for_data(Duration::from_millis(1000)) {
            println!("Waiting for iRacing connection...");
        }
        if let Some(data) = client.data() {
            for (key, val) in data.vars {
                println!("{}: {:?}", key, val);
            }
        }
        if let Some(session_info) = client.session_info_raw() {
            println!("Session info: {}", session_info);
        }
        println!("Connected!");
        while client.connected() {
            if client.wait_for_data(Duration::from_millis(16)) {
                println!("Received new data");
            } else {
                println!("Waiting for further data");
            }
        }
    }
}
