use crate::{Moment, Simetry};
use anyhow::{Context, Result};
use hyper::body::Buf;
use hyper::client::HttpConnector;
use hyper::{Client as HyperClient, Uri};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::time::Duration;
use time::serde::iso8601;
use time::OffsetDateTime;
use tokio::time::timeout;
use uom::si::angular_velocity::revolution_per_minute;
use uom::si::f64::{AngularVelocity, Velocity};
use uom::si::velocity::kilometer_per_hour;

pub const DEFAULT_URI: &str = "http://localhost:25555/api/ets2/telemetry";

/// Client for Euro Truck Simulator 2 and American Truck Simulator
///
/// Uses https://github.com/Funbit/ets2-telemetry-server to query JSON data
pub struct Client {
    name: String,
    client: HyperClient<HttpConnector>,
    uri: Uri,
}

#[async_trait::async_trait]
impl Simetry for Client {
    fn name(&self) -> &str {
        &self.name
    }

    async fn next_moment(&mut self) -> Option<Box<dyn Moment + Send + Sync + 'static>> {
        let data = timeout(Duration::from_secs(2), self.query())
            .await
            .ok()?
            .ok()?;
        if data.game.game_name.as_ref()? != &self.name {
            return None;
        }
        Some(Box::new(data))
    }
}

impl Client {
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
            client: HyperClient::new(),
            uri: uri.parse()?,
        };
        let telemetry = slf.query().await?;
        slf.name = telemetry
            .game
            .game_name
            .context("The sim is not yet running")?;
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
pub struct Navigation {
    #[serde(rename = "estimatedTime", with = "iso8601")]
    pub estimated_time: OffsetDateTime,
    #[serde(rename = "estimatedDistance")]
    pub estimated_distance: i64,
    #[serde(rename = "speedLimit")]
    pub speed_limit: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Job {
    pub income: i64,
    #[serde(rename = "deadlineTime", with = "iso8601")]
    pub deadline_time: OffsetDateTime,
    #[serde(rename = "remainingTime", with = "iso8601")]
    pub remaining_time: OffsetDateTime,
    #[serde(rename = "sourceCity")]
    pub source_city: String,
    #[serde(rename = "sourceCompany")]
    pub source_company: String,
    #[serde(rename = "destinationCity")]
    pub destination_city: String,
    #[serde(rename = "destinationCompany")]
    pub destination_company: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Trailer {
    pub attached: bool,
    pub id: String,
    pub name: String,
    pub mass: f64,
    pub wear: f64,
    pub placement: Placement,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Placement {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub heading: f64,
    pub pitch: f64,
    pub roll: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Truck {
    pub id: String,
    pub make: String,
    pub model: String,
    pub speed: f64,
    #[serde(rename = "cruiseControlSpeed")]
    pub cruise_control_speed: f64,
    #[serde(rename = "cruiseControlOn")]
    pub cruise_control_on: bool,
    pub odometer: f64,
    pub gear: i64,
    #[serde(rename = "displayedGear")]
    pub displayed_gear: i64,
    #[serde(rename = "forwardGears")]
    pub forward_gears: i64,
    #[serde(rename = "reverseGears")]
    pub reverse_gears: i64,
    #[serde(rename = "shifterType")]
    pub shifter_type: String,
    #[serde(rename = "engineRpm")]
    pub engine_rpm: f64,
    #[serde(rename = "engineRpmMax")]
    pub engine_rpm_max: f64,
    pub fuel: f64,
    #[serde(rename = "fuelCapacity")]
    pub fuel_capacity: f64,
    #[serde(rename = "fuelAverageConsumption")]
    pub fuel_average_consumption: f64,
    #[serde(rename = "fuelWarningFactor")]
    pub fuel_warning_factor: f64,
    #[serde(rename = "fuelWarningOn")]
    pub fuel_warning_on: bool,
    #[serde(rename = "wearEngine")]
    pub wear_engine: f64,
    #[serde(rename = "wearTransmission")]
    pub wear_transmission: f64,
    #[serde(rename = "wearCabin")]
    pub wear_cabin: f64,
    #[serde(rename = "wearChassis")]
    pub wear_chassis: f64,
    #[serde(rename = "wearWheels")]
    pub wear_wheels: f64,
    #[serde(rename = "userSteer")]
    pub user_steer: f64,
    #[serde(rename = "userThrottle")]
    pub user_throttle: f64,
    #[serde(rename = "userBrake")]
    pub user_brake: f64,
    #[serde(rename = "userClutch")]
    pub user_clutch: f64,
    #[serde(rename = "gameSteer")]
    pub game_steer: f64,
    #[serde(rename = "gameThrottle")]
    pub game_throttle: f64,
    #[serde(rename = "gameBrake")]
    pub game_brake: f64,
    #[serde(rename = "gameClutch")]
    pub game_clutch: f64,
    #[serde(rename = "shifterSlot")]
    pub shifter_slot: i64,
    #[serde(rename = "engineOn")]
    pub engine_on: bool,
    #[serde(rename = "electricOn")]
    pub electric_on: bool,
    #[serde(rename = "wipersOn")]
    pub wipers_on: bool,
    #[serde(rename = "retarderBrake")]
    pub retarder_brake: i64,
    #[serde(rename = "retarderStepCount")]
    pub retarder_step_count: i64,
    #[serde(rename = "parkBrakeOn")]
    pub park_brake_on: bool,
    #[serde(rename = "motorBrakeOn")]
    pub motor_brake_on: bool,
    #[serde(rename = "brakeTemperature")]
    pub brake_temperature: f64,
    pub adblue: f64,
    #[serde(rename = "adblueCapacity")]
    pub adblue_capacity: f64,
    #[serde(rename = "adblueAverageConsumption")]
    pub adblue_average_consumption: f64,
    #[serde(rename = "adblueWarningOn")]
    pub adblue_warning_on: bool,
    #[serde(rename = "airPressure")]
    pub air_pressure: f64,
    #[serde(rename = "airPressureWarningOn")]
    pub air_pressure_warning_on: bool,
    #[serde(rename = "airPressureWarningValue")]
    pub air_pressure_warning_value: f64,
    #[serde(rename = "airPressureEmergencyOn")]
    pub air_pressure_emergency_on: bool,
    #[serde(rename = "airPressureEmergencyValue")]
    pub air_pressure_emergency_value: f64,
    #[serde(rename = "oilTemperature")]
    pub oil_temperature: f64,
    #[serde(rename = "oilPressure")]
    pub oil_pressure: f64,
    #[serde(rename = "oilPressureWarningOn")]
    pub oil_pressure_warning_on: bool,
    #[serde(rename = "oilPressureWarningValue")]
    pub oil_pressure_warning_value: f64,
    #[serde(rename = "waterTemperature")]
    pub water_temperature: f64,
    #[serde(rename = "waterTemperatureWarningOn")]
    pub water_temperature_warning_on: bool,
    #[serde(rename = "waterTemperatureWarningValue")]
    pub water_temperature_warning_value: f64,
    #[serde(rename = "batteryVoltage")]
    pub battery_voltage: f64,
    #[serde(rename = "batteryVoltageWarningOn")]
    pub battery_voltage_warning_on: bool,
    #[serde(rename = "batteryVoltageWarningValue")]
    pub battery_voltage_warning_value: f64,
    #[serde(rename = "lightsDashboardValue")]
    pub lights_dashboard_value: f64,
    #[serde(rename = "lightsDashboardOn")]
    pub lights_dashboard_on: bool,
    #[serde(rename = "blinkerLeftActive")]
    pub blinker_left_active: bool,
    #[serde(rename = "blinkerRightActive")]
    pub blinker_right_active: bool,
    #[serde(rename = "blinkerLeftOn")]
    pub blinker_left_on: bool,
    #[serde(rename = "blinkerRightOn")]
    pub blinker_right_on: bool,
    #[serde(rename = "lightsParkingOn")]
    pub lights_parking_on: bool,
    #[serde(rename = "lightsBeamLowOn")]
    pub lights_beam_low_on: bool,
    #[serde(rename = "lightsBeamHighOn")]
    pub lights_beam_high_on: bool,
    #[serde(rename = "lightsAuxFrontOn")]
    pub lights_aux_front_on: bool,
    #[serde(rename = "lightsAuxRoofOn")]
    pub lights_aux_roof_on: bool,
    #[serde(rename = "lightsBeaconOn")]
    pub lights_beacon_on: bool,
    #[serde(rename = "lightsBrakeOn")]
    pub lights_brake_on: bool,
    #[serde(rename = "lightsReverseOn")]
    pub lights_reverse_on: bool,
    pub placement: Placement,
    pub acceleration: Vector,
    pub head: Vector,
    pub cabin: Vector,
    pub hook: Vector,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Game {
    pub connected: bool,
    #[serde(rename = "gameName")]
    pub game_name: Option<String>,
    pub paused: bool,
    #[serde(with = "iso8601")]
    pub time: OffsetDateTime,
    #[serde(rename = "timeScale")]
    pub time_scale: f64,
    #[serde(rename = "nextRestStopTime", with = "iso8601")]
    pub next_rest_stop_time: OffsetDateTime,
    pub version: String,
    #[serde(rename = "telemetryPluginVersion")]
    pub telemetry_plugin_version: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimState {
    pub game: Game,
    pub truck: Truck,
    pub trailer: Trailer,
    pub job: Job,
    pub navigation: Navigation,
}

impl Moment for SimState {
    fn vehicle_gear(&self) -> Option<i8> {
        // Maybe use displayed_gear for this?
        Some(self.truck.gear as i8)
    }

    fn vehicle_velocity(&self) -> Option<Velocity> {
        Some(Velocity::new::<kilometer_per_hour>(self.truck.speed))
    }

    fn vehicle_engine_rotation_speed(&self) -> Option<AngularVelocity> {
        Some(AngularVelocity::new::<revolution_per_minute>(
            self.truck.engine_rpm,
        ))
    }

    fn vehicle_max_engine_rotation_speed(&self) -> Option<AngularVelocity> {
        Some(AngularVelocity::new::<revolution_per_minute>(
            self.truck.engine_rpm_max,
        ))
    }

    fn vehicle_brand_id(&self) -> Option<Cow<str>> {
        Some(self.truck.make.as_str().into())
    }

    fn vehicle_model_id(&self) -> Option<Cow<str>> {
        Some(self.truck.model.as_str().into())
    }

    fn is_left_turn_indicator_on(&self) -> Option<bool> {
        Some(self.truck.blinker_left_active)
    }

    fn is_right_turn_indicator_on(&self) -> Option<bool> {
        Some(self.truck.blinker_right_active)
    }

    fn is_ignition_on(&self) -> Option<bool> {
        Some(self.truck.electric_on)
    }
}
