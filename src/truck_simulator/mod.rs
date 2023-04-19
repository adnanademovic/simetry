/// Client for Euro Truck Simulator 2 and American Truck Simulator
///
/// Uses https://github.com/Funbit/ets2-telemetry-server to query JSON data
use anyhow::Result;
use hyper::body::Buf;
use hyper::Client;
use serde_json::Value;

pub const DEFAULT_URI: &str = "http://localhost:25555/api/ets2/telemetry";

pub async fn query(uri: &str) -> Result<Value> {
    let client = Client::new();
    let response = client.get(uri.parse()?).await?;
    let bytes = hyper::body::to_bytes(response.into_body()).await?;
    let data = serde_json::from_reader(bytes.reader())?;
    Ok(data)
}
