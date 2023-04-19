use crate::{BasicTelemetry, Moment, RacingFlags, Simetry};
use anyhow::Result;
use hyper::body::Buf;
use hyper::client::HttpConnector;
use hyper::{Client, Uri};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::timeout;
use uom::si::f64::AngularVelocity;

pub const DEFAULT_ADDRESS: &str = "0.0.0.0:25055";
pub const DEFAULT_URI: &str = "http://localhost:25055/";

#[derive(Debug)]
pub struct GenericHttpClient {
    name: String,
    client: Client<HttpConnector>,
    uri: Uri,
}

impl GenericHttpClient {
    pub async fn connect(uri: &str) -> Result<Self> {
        let mut slf = Self {
            name: "".to_string(),
            client: Client::new(),
            uri: uri.parse()?,
        };
        let sim_state = slf.query().await?;
        slf.name = sim_state.name;
        Ok(slf)
    }

    pub async fn query(&self) -> Result<SimState> {
        let response = self.client.get(self.uri.clone()).await?;
        let bytes = hyper::body::to_bytes(response.into_body()).await?;
        let data = serde_json::from_reader(bytes.reader())?;
        Ok(data)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimState {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub car_left: bool,
    #[serde(default)]
    pub car_right: bool,
    #[serde(default)]
    pub basic_telemetry: Option<BasicTelemetry>,
    #[serde(default)]
    pub shift_point: Option<AngularVelocity>,
    #[serde(default)]
    pub flags: RacingFlags,
    #[serde(default)]
    pub car_model_id: Option<String>,
    #[serde(default)]
    pub ignition_on: bool,
    #[serde(default)]
    pub starter_on: bool,
}

#[async_trait::async_trait]
impl Simetry for GenericHttpClient {
    fn name(&self) -> &str {
        &self.name
    }

    async fn next_moment(&mut self) -> Option<Box<dyn Moment>> {
        let data = timeout(Duration::from_secs(2), self.query())
            .await
            .ok()?
            .ok()?;
        if data.name != self.name {
            return None;
        }
        Some(Box::new(data))
    }
}

impl Moment for SimState {
    fn car_left(&self) -> bool {
        self.car_left
    }

    fn car_right(&self) -> bool {
        self.car_right
    }

    fn basic_telemetry(&self) -> Option<BasicTelemetry> {
        self.basic_telemetry.clone()
    }

    fn shift_point(&self) -> Option<AngularVelocity> {
        self.shift_point
    }

    fn flags(&self) -> RacingFlags {
        self.flags.clone()
    }

    fn car_model_id(&self) -> Option<String> {
        self.car_model_id.clone()
    }

    fn ignition_on(&self) -> bool {
        self.ignition_on
    }

    fn starter_on(&self) -> bool {
        self.starter_on
    }
}
