pub use racing_flags::RacingFlags;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::select;
use uom::si::f64::{AngularVelocity, Velocity};

pub mod assetto_corsa;
pub mod assetto_corsa_competizione;
pub mod dirt_rally_2;
pub mod generic_http;
pub mod iracing;
mod racing_flags;
pub mod rfactor_2;
pub mod truck_simulator;
mod windows_util;

#[inline]
fn unhandled_default<T: Default>() -> T {
    unhandled(T::default())
}

#[inline]
fn unhandled<T>(value: T) -> T {
    value
}

#[async_trait::async_trait]
pub trait Simetry {
    fn name(&self) -> &str;

    async fn next_moment(&mut self) -> Option<Box<dyn Moment>>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimetryConnectionBuilder {
    pub generic_http_uri: String,
    pub truck_simulator_uri: String,
    pub dirt_rally_2_uri: String,
    pub retry_delay: Duration,
}

impl Default for SimetryConnectionBuilder {
    fn default() -> Self {
        Self {
            generic_http_uri: generic_http::DEFAULT_URI.to_string(),
            truck_simulator_uri: truck_simulator::DEFAULT_URI.to_string(),
            dirt_rally_2_uri: dirt_rally_2::Client::DEFAULT_URI.to_string(),
            retry_delay: Duration::from_secs(5),
        }
    }
}

impl SimetryConnectionBuilder {
    pub async fn connect(self) -> Box<dyn Simetry> {
        let retry_delay = self.retry_delay;
        let iracing_future = iracing::Client::connect(retry_delay);
        let assetto_corsa_future = assetto_corsa::Client::connect(retry_delay);
        let assetto_corsa_competizione_future =
            assetto_corsa_competizione::Client::connect(retry_delay);
        let rfactor_2_future = rfactor_2::Client::connect();
        let dirt_rally_2_future =
            dirt_rally_2::Client::connect(&self.dirt_rally_2_uri, retry_delay);
        let generic_http_future =
            generic_http::GenericHttpClient::connect(&self.generic_http_uri, retry_delay);
        let truck_simulator_future =
            truck_simulator::TruckSimulatorClient::connect(&self.truck_simulator_uri, retry_delay);

        select! {
            x = iracing_future => Box::new(x),
            x = assetto_corsa_future => Box::new(x),
            x = assetto_corsa_competizione_future => Box::new(x),
            x = rfactor_2_future => Box::new(x),
            x = dirt_rally_2_future => Box::new(x),
            x = generic_http_future => Box::new(x),
            x = truck_simulator_future => Box::new(x),
        }
    }
}

#[inline]
pub async fn connect() -> Box<dyn Simetry> {
    SimetryConnectionBuilder::default().connect().await
}

pub trait Moment {
    fn car_left(&self) -> bool;
    fn car_right(&self) -> bool;
    fn basic_telemetry(&self) -> Option<BasicTelemetry>;
    fn shift_point(&self) -> Option<AngularVelocity>;
    fn flags(&self) -> RacingFlags;
    fn car_model_id(&self) -> Option<String>;
    fn ignition_on(&self) -> bool;
    fn starter_on(&self) -> bool;
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BasicTelemetry {
    pub gear: i8,
    pub speed: Velocity,
    pub engine_rotation_speed: AngularVelocity,
    pub max_engine_rotation_speed: AngularVelocity,
    pub pit_limiter_engaged: bool,
    pub in_pit_lane: bool,
}
