use simetry::iracing_basic_solution::Client;
use std::time::Duration;

fn main() {
    let mut client = Client::new();
    loop {
        while !client.wait_for_data(Duration::from_millis(1000)) {
            println!("Waiting for iRacing connection...");
        }
        println!("Connected!");
        while client.connected() {
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
