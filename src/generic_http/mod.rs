use crate::{Moment, Pedals, RacingFlags, Simetry};
use anyhow::Result;
use hyper::body::Buf;
use hyper::client::HttpConnector;
use hyper::{Client, Uri};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::time::Duration;
use tokio::time::timeout;
use uom::si::f64::{AngularVelocity, Velocity};

pub const DEFAULT_ADDRESS: &str = "0.0.0.0:25055";
pub const DEFAULT_URI: &str = "http://localhost:25055/";

#[derive(Debug)]
pub struct GenericHttpClient {
    name: String,
    client: Client<HttpConnector>,
    uri: Uri,
}

impl GenericHttpClient {
    pub async fn connect(uri: &str, retry_delay: Duration) -> Self {
        loop {
            if let Ok(client) = Self::try_connect(uri).await {
                return client;
            }
            tokio::time::sleep(retry_delay).await;
        }
    }

    pub async fn try_connect(uri: &str) -> Result<Self> {
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
    pub vehicle_left: Option<bool>,
    #[serde(default)]
    pub vehicle_right: Option<bool>,
    #[serde(default)]
    pub gear: Option<i8>,
    #[serde(default)]
    pub speed: Option<Velocity>,
    #[serde(default)]
    pub engine_rotation_speed: Option<AngularVelocity>,
    #[serde(default)]
    pub max_engine_rotation_speed: Option<AngularVelocity>,
    #[serde(default)]
    pub pit_limiter_engaged: Option<bool>,
    #[serde(default)]
    pub in_pit_lane: Option<bool>,
    #[serde(default)]
    pub shift_point: Option<AngularVelocity>,
    #[serde(default)]
    pub flags: Option<RacingFlags>,
    #[serde(default)]
    pub vehicle_brand_id: Option<String>,
    #[serde(default)]
    pub vehicle_model_id: Option<String>,
    #[serde(default)]
    pub vehicle_unique_id: Option<String>,
    #[serde(default)]
    pub left_turn_indicator_on: Option<bool>,
    #[serde(default)]
    pub right_turn_indicator_on: Option<bool>,
    #[serde(default)]
    pub hazard_indicator_on: Option<bool>,
    #[serde(default)]
    pub ignition_on: Option<bool>,
    #[serde(default)]
    pub starter_on: Option<bool>,
    #[serde(default)]
    pub pedals: Option<Pedals>,
    #[serde(default)]
    pub pedals_raw: Option<Pedals>,
}

#[async_trait::async_trait]
impl Simetry for GenericHttpClient {
    fn name(&self) -> &str {
        &self.name
    }

    async fn next_moment(&mut self) -> Option<Box<dyn Moment + Send + Sync + 'static>> {
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
    fn vehicle_gear(&self) -> Option<i8> {
        self.gear
    }

    fn vehicle_velocity(&self) -> Option<Velocity> {
        self.speed
    }

    fn vehicle_engine_rotation_speed(&self) -> Option<AngularVelocity> {
        self.engine_rotation_speed
    }

    fn vehicle_max_engine_rotation_speed(&self) -> Option<AngularVelocity> {
        self.max_engine_rotation_speed
    }

    fn is_pit_limiter_engaged(&self) -> Option<bool> {
        self.pit_limiter_engaged
    }

    fn is_vehicle_in_pit_lane(&self) -> Option<bool> {
        self.in_pit_lane
    }

    fn is_vehicle_left(&self) -> Option<bool> {
        self.vehicle_left
    }

    fn is_vehicle_right(&self) -> Option<bool> {
        self.vehicle_right
    }

    fn shift_point(&self) -> Option<AngularVelocity> {
        self.shift_point
    }

    fn flags(&self) -> Option<RacingFlags> {
        self.flags.clone()
    }

    fn vehicle_brand_id(&self) -> Option<Cow<str>> {
        Some(self.vehicle_brand_id.as_ref()?.into())
    }

    fn vehicle_model_id(&self) -> Option<Cow<str>> {
        Some(self.vehicle_model_id.as_ref()?.into())
    }

    fn vehicle_unique_id(&self) -> Option<Cow<str>> {
        Some(self.vehicle_unique_id.as_ref()?.into())
    }

    fn is_left_turn_indicator_on(&self) -> Option<bool> {
        self.left_turn_indicator_on
    }

    fn is_right_turn_indicator_on(&self) -> Option<bool> {
        self.right_turn_indicator_on
    }

    fn is_hazard_indicator_on(&self) -> Option<bool> {
        self.hazard_indicator_on
    }

    fn is_ignition_on(&self) -> Option<bool> {
        self.ignition_on
    }

    fn is_starter_on(&self) -> Option<bool> {
        self.starter_on
    }

    fn pedals(&self) -> Option<Pedals> {
        self.pedals.clone()
    }

    fn pedals_raw(&self) -> Option<Pedals> {
        self.pedals_raw.clone()
    }
}
