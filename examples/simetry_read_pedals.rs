#[tokio::main]
async fn main() {
    loop {
        println!("Starting connection...");
        let mut client = simetry::connect().await;
        println!("Connected!");
        while let Some(moment) = client.next_moment().await {
            let pedals = moment.pedals().unwrap_or_default();
            let pedals_raw = moment.pedals_raw().unwrap_or_default();
            println!(
                "game: T: {:5.3} B: {:5.3} C: {:5.3}, raw: T: {:5.3} B: {:5.3} C: {:5.3}",
                pedals.throttle,
                pedals.brake,
                pedals.clutch,
                pedals_raw.throttle,
                pedals_raw.brake,
                pedals_raw.clutch,
            );
        }
        println!("Connection finished!");
    }
}
