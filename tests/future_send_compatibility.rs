#[tokio::test]
async fn compiles_if_send_is_valid() {
    let task = tokio::spawn(async move {
        let mut client = simetry::connect().await;
        let moment = client.next_moment().await.unwrap();
        println!("{}", moment.vehicle_gear().unwrap_or_default());
    });
    task.abort();
}
