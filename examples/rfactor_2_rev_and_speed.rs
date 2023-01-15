use simetry::rfactor_2::Client;

#[tokio::main]
async fn main() {
    loop {
        println!("Starting connection to rFactor 2...");
        let mut client = Client::connect().await;
        println!("Connected!");
        while let Some(sim_state) = client.next_sim_state().await {
            println!("== Cars ==");
            for (idx, vehicle) in sim_state.telemetry.vehicles.iter().enumerate() {
                let rpm = f64::round(vehicle.engine_rpm);
                let speed = f64::round(vehicle.local_vel.length() * 3.6);
                println!("Car {idx}: {speed} km/h @ {rpm} RPM");
            }
        }
        println!("Connection finished!");
    }
}
