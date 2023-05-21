use simetry::dirt_rally_2;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let client =
        dirt_rally_2::Client::connect(dirt_rally_2::Client::DEFAULT_URI, Duration::from_secs(1))
            .await;

    while let Ok(res) = tokio::time::timeout(Duration::from_secs(2), client.next_sim_state()).await
    {
        let sim_state = res.unwrap();
        println!("{sim_state:#?}");
    }
}
