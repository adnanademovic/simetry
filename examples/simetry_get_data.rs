use uom::si::angular_velocity::revolution_per_minute;
use uom::si::velocity::kilometer_per_hour;

#[tokio::main]
async fn main() {
    loop {
        println!("Starting connection...");
        let mut client = simetry::connect().await;
        println!("Connected!");
        while let Some(moment) = client.next_moment().await {
            if let Some(telemetry) = moment.basic_telemetry() {
                println!(
                    "In {} gear, {} km/h @ {} RPM",
                    telemetry.gear,
                    telemetry.speed.get::<kilometer_per_hour>().round(),
                    telemetry
                        .engine_rotation_speed
                        .get::<revolution_per_minute>()
                        .round(),
                );
            }
        }
        println!("Connection finished!");
    }
}
