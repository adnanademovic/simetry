use uom::si::angular_velocity::revolution_per_minute;
use uom::si::velocity::kilometer_per_hour;

#[tokio::main]
async fn main() {
    loop {
        println!("Starting connection...");
        let mut client = simetry::connect().await;
        println!("Connected!");
        while let Some(moment) = client.next_moment().await {
            println!(
                "In {:?} gear, {:?} km/h @ {:?} RPM, shift RPM: {:?}",
                moment.vehicle_gear(),
                moment
                    .vehicle_velocity()
                    .map(|v| v.get::<kilometer_per_hour>().round()),
                moment
                    .vehicle_engine_rotation_speed()
                    .map(|v| v.get::<revolution_per_minute>().round()),
                moment
                    .shift_point()
                    .map(|v| v.get::<revolution_per_minute>().round()),
            );
        }
        println!("Connection finished!");
    }
}
