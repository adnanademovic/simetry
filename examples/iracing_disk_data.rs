use simetry::iracing_basic_solution::DiskClient;
use std::env;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut client =
        DiskClient::open(env::args().nth(1).expect("Filename argument required")).unwrap();
    for (key, val) in client.vars() {
        println!("{}: {:?}", key, val);
    }
    println!("Session info: {:?}", client.session_info());
    sleep(Duration::from_millis(500));
    while client.next_data().is_ok() {
        println!("Received new data");
        sleep(Duration::from_millis(16));
    }
}
