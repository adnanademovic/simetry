pub use racing_flags::RacingFlags;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::time::Duration;
use tokio::select;
use uom::si::f64::{AngularVelocity, Velocity};

pub mod assetto_corsa;
pub mod assetto_corsa_competizione;
pub mod dirt_rally_2;
#[cfg(feature = "unstable_generic_http_client")]
pub mod generic_http;
pub mod iracing;
pub mod raceroom_racing_experience;
mod racing_flags;
pub mod rfactor_2;
pub mod truck_simulator;
mod windows_util;

/// Sim that we can connect to via the common [`connect`] function.
#[async_trait::async_trait]
pub trait Simetry {
    /// Name of the sim we are connected to.
    fn name(&self) -> &str;

    /// Waits for the next reading of data from the sim and returns it.
    ///
    /// A `None` value means that the connection is done, similar to an iterator.
    async fn next_moment(&mut self) -> Option<Box<dyn Moment + Send + Sync + 'static>>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimetryConnectionBuilder {
    #[cfg(feature = "unstable_generic_http_client")]
    generic_http_uri: String,
    dirt_rally_2_uri: String,
    retry_delay: Duration,
}

impl Default for SimetryConnectionBuilder {
    fn default() -> Self {
        Self {
            #[cfg(feature = "unstable_generic_http_client")]
            generic_http_uri: generic_http::DEFAULT_URI.to_string(),
            dirt_rally_2_uri: dirt_rally_2::Client::DEFAULT_URI.to_string(),
            retry_delay: Duration::from_secs(5),
        }
    }
}

impl SimetryConnectionBuilder {
    #[cfg(feature = "unstable_generic_http_client")]
    pub fn generic_http_uri(mut self, uri: String) -> Self {
        self.generic_http_uri = uri;
        self
    }

    pub fn dirt_rally_2_uri(mut self, uri: String) -> Self {
        self.dirt_rally_2_uri = uri;
        self
    }

    pub fn retry_delay(mut self, delay: Duration) -> Self {
        self.retry_delay = delay;
        self
    }

    pub async fn connect(self) -> Box<dyn Simetry + Send + Sync + 'static> {
        let retry_delay = self.retry_delay;
        let iracing_future = iracing::Client::connect(retry_delay);
        let assetto_corsa_future = assetto_corsa::Client::connect(retry_delay);
        let assetto_corsa_competizione_future =
            assetto_corsa_competizione::Client::connect(retry_delay);
        let raceroom_racing_experience_future =
            raceroom_racing_experience::Client::connect(retry_delay);
        let rfactor_2_future = rfactor_2::Client::connect();
        let dirt_rally_2_future =
            dirt_rally_2::Client::connect(&self.dirt_rally_2_uri, retry_delay);
        #[cfg(feature = "unstable_generic_http_client")]
        let generic_http_future =
            generic_http::GenericHttpClient::connect(&self.generic_http_uri, retry_delay);
        #[cfg(not(feature = "unstable_generic_http_client"))]
        let generic_http_future = never_resolved();
        let truck_simulator_future = truck_simulator::Client::connect(retry_delay);

        select! {
            x = iracing_future => Box::new(x),
            x = assetto_corsa_future => Box::new(x),
            x = assetto_corsa_competizione_future => Box::new(x),
            x = raceroom_racing_experience_future => Box::new(x),
            x = rfactor_2_future => Box::new(x),
            x = dirt_rally_2_future => Box::new(x),
            x = generic_http_future => Box::new(x),
            x = truck_simulator_future => Box::new(x),
        }
    }
}

#[cfg(not(feature = "unstable_generic_http_client"))]
async fn never_resolved() -> iracing::Client {
    loop {
        tokio::time::sleep(Duration::from_secs(1_000_000_000)).await;
    }
}

/// Connect to any running sim that is supported.
#[inline]
pub async fn connect() -> Box<dyn Simetry + Send + Sync + 'static> {
    SimetryConnectionBuilder::default().connect().await
}

/// Generic support for any sim by providing processed data for most common data-points.
pub trait Moment {
    fn vehicle_gear(&self) -> Option<i8> {
        None
    }

    fn vehicle_velocity(&self) -> Option<Velocity> {
        None
    }

    fn vehicle_engine_rotation_speed(&self) -> Option<AngularVelocity> {
        None
    }

    fn vehicle_max_engine_rotation_speed(&self) -> Option<AngularVelocity> {
        None
    }

    fn is_pit_limiter_engaged(&self) -> Option<bool> {
        None
    }

    fn is_vehicle_in_pit_lane(&self) -> Option<bool> {
        None
    }

    /// Check if there is a vehicle to the left of the driver.
    fn is_vehicle_left(&self) -> Option<bool> {
        None
    }

    /// Check if there is a vehicle to the right of the driver.
    fn is_vehicle_right(&self) -> Option<bool> {
        None
    }

    fn shift_point(&self) -> Option<AngularVelocity> {
        None
    }

    fn flags(&self) -> Option<RacingFlags> {
        None
    }

    /// ID that should be consistent to all vehicles of the same brand in the specific sim.
    ///
    /// The same brand can have different IDs in different sims. A car brand called "Lemon"
    /// could be represented in many ways: "lemon", "LEMON", "Lemon", "Lemon Car Company", "57" etc.
    ///
    /// This interface basically allows for easily correlating cars of the same brand in a sim.
    fn vehicle_brand_id(&self) -> Option<Cow<str>> {
        None
    }

    /// ID that describes a specific model of a car in the specific sim.
    ///
    /// This interface does not not uniquely identify a specific brand and model, because two
    /// different brands may have a model with the same name. For unique identifiers, use
    /// [`Moment::vehicle_unique_id`].
    fn vehicle_model_id(&self) -> Option<Cow<str>> {
        None
    }

    /// ID that uniquely identifies the current vehicle brand and model.
    ///
    /// If you want to provide behavior for a specific vehicle brand and model,
    /// this property is the right choice.
    fn vehicle_unique_id(&self) -> Option<Cow<str>> {
        let brand = self.vehicle_brand_id()?;
        let model = self.vehicle_model_id()?;
        Some(format!("{brand}|{model}").into())
    }

    /// Left turn indicator is enabled.
    ///
    /// This does not specify whether the blinker light is on or off, just that it's blinking.
    fn is_left_turn_indicator_on(&self) -> Option<bool> {
        None
    }

    /// Right turn indicator is enabled.
    ///
    /// This does not specify whether the blinker light is on or off, just that it's blinking.
    fn is_right_turn_indicator_on(&self) -> Option<bool> {
        None
    }

    /// Hazard indicator is enabled.
    ///
    /// This does not specify whether the lights are on or off, just that it's blinking.
    fn is_hazard_indicator_on(&self) -> Option<bool> {
        Some(self.is_left_turn_indicator_on()? && self.is_right_turn_indicator_on()?)
    }

    /// Check if the ignition is on.
    fn is_ignition_on(&self) -> Option<bool> {
        None
    }

    /// Check if the starter motor is engaged.
    fn is_starter_on(&self) -> Option<bool> {
        None
    }

    /// Pedal input as read by the game.
    ///
    /// This often includes modifications caused by assists.
    fn pedals(&self) -> Option<Pedals> {
        None
    }

    /// Pedal input directly from the inputs.
    fn pedals_raw(&self) -> Option<Pedals> {
        self.pedals()
    }
}

/// Percentage values of pedal inputs.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Pedals {
    pub throttle: f64,
    pub brake: f64,
    pub clutch: f64,
}
