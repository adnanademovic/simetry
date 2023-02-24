use crate::iracing::CarPositions;
use std::future::Future;
use std::time::Duration;
use tokio::select;
use uom::si::angular_velocity::revolution_per_minute;
use uom::si::f64::{AngularVelocity, Velocity};
use uom::si::velocity::{kilometer_per_hour, meter_per_second};

pub mod assetto_corsa;
pub mod assetto_corsa_competizione;
pub mod iracing;
pub mod rfactor_2;
mod windows_util;

enum SimetrySource {
    IRacing(iracing::Client),
    AssettoCorsa(assetto_corsa::Client),
    AssettoCorsaCompetizione(assetto_corsa_competizione::Client),
    RFactor2(rfactor_2::Client),
}

pub struct Simetry {
    inner: SimetrySource,
}

async fn loop_until_success<R, T, F>(f: F, delay: Duration) -> R
where
    T: Future<Output = anyhow::Result<R>>,
    F: Fn() -> T,
{
    loop {
        if let Ok(v) = f().await {
            return v;
        }
        tokio::time::sleep(delay).await;
    }
}

impl Simetry {
    pub async fn connect() -> Self {
        let retry_delay = Duration::from_secs(5);
        let iracing_future = loop_until_success(iracing::Client::connect, retry_delay);
        let assetto_corsa_future = loop_until_success(assetto_corsa::Client::connect, retry_delay);
        let assetto_corsa_competizione_future =
            loop_until_success(assetto_corsa_competizione::Client::connect, retry_delay);
        let rfactor_2_future = rfactor_2::Client::connect();
        let inner = select! {
            x = iracing_future => SimetrySource::IRacing(x),
            x = assetto_corsa_future => SimetrySource::AssettoCorsa(x),
            x = assetto_corsa_competizione_future => SimetrySource::AssettoCorsaCompetizione(x),
            x = rfactor_2_future => SimetrySource::RFactor2(x),
        };
        Self { inner }
    }

    pub async fn next_moment(&mut self) -> Option<Moment> {
        Some(Moment {
            inner: match &mut self.inner {
                SimetrySource::IRacing(v) => MomentSource::IRacing(v.next_sim_state().await?),
                SimetrySource::AssettoCorsa(v) => {
                    MomentSource::AssettoCorsa(v.next_sim_state().await?)
                }
                SimetrySource::AssettoCorsaCompetizione(v) => {
                    MomentSource::AssettoCorsaCompetizione(v.next_sim_state().await?)
                }
                SimetrySource::RFactor2(v) => {
                    let sim_state = v.next_sim_state().await?;
                    let player_id = sim_state
                        .scoring
                        .vehicles
                        .iter()
                        .find(|v| v.is_player != 0)
                        .map(|v| v.id);
                    MomentSource::RFactor2 {
                        sim_state,
                        player_id,
                    }
                }
            },
        })
    }
}

pub async fn connect() -> Simetry {
    Simetry::connect().await
}

enum MomentSource {
    IRacing(iracing::SimState),
    AssettoCorsa(assetto_corsa::SimState),
    AssettoCorsaCompetizione(assetto_corsa_competizione::SimState),
    RFactor2 {
        sim_state: rfactor_2::SimState,
        player_id: Option<i32>,
    },
}

pub struct Moment {
    inner: MomentSource,
}

pub struct BasicTelemetry {
    pub gear: i8,
    pub speed: Velocity,
    pub engine_rotation_speed: AngularVelocity,
    pub max_engine_rotation_speed: AngularVelocity,
    pub pit_limiter_engaged: bool,
    pub in_pit_lane: bool,
}

impl Moment {
    pub fn car_left(&self) -> bool {
        match &self.inner {
            MomentSource::IRacing(v) => v
                .read_name("CarLeftRight")
                .unwrap_or(CarPositions::Off)
                .car_left(),
            MomentSource::AssettoCorsa(_)
            | MomentSource::AssettoCorsaCompetizione(_)
            | MomentSource::RFactor2 { .. } => false,
        }
    }

    pub fn car_right(&self) -> bool {
        match &self.inner {
            MomentSource::IRacing(v) => v
                .read_name("CarLeftRight")
                .unwrap_or(CarPositions::Off)
                .car_right(),
            MomentSource::AssettoCorsa(_)
            | MomentSource::AssettoCorsaCompetizione(_)
            | MomentSource::RFactor2 { .. } => false,
        }
    }

    pub fn basic_telemetry(&self) -> Option<BasicTelemetry> {
        Some(match &self.inner {
            MomentSource::IRacing(v) => BasicTelemetry {
                gear: v.read_name("Gear").unwrap_or(0i32) as i8,
                speed: Velocity::new::<meter_per_second>(v.read_name("Speed").unwrap_or(0.0)),
                engine_rotation_speed: AngularVelocity::new::<revolution_per_minute>(
                    v.read_name("RPM").unwrap_or(0.0),
                ),
                max_engine_rotation_speed: AngularVelocity::new::<revolution_per_minute>(
                    v.session_info()["DriverInfo"]["DriverCarRedLine"]
                        .as_f64()
                        .unwrap_or(f64::INFINITY),
                ),
                pit_limiter_engaged: v.read_name("dcPitSpeedLimiterToggle").unwrap_or(false),
                in_pit_lane: v.read_name("OnPitRoad").unwrap_or(false),
            },
            MomentSource::AssettoCorsa(v) => BasicTelemetry {
                gear: (v.physics.gear - 1) as i8,
                speed: Velocity::new::<kilometer_per_hour>(v.physics.speed_kmh as f64),
                engine_rotation_speed: AngularVelocity::new::<revolution_per_minute>(
                    v.physics.rpm as f64,
                ),
                max_engine_rotation_speed: AngularVelocity::new::<revolution_per_minute>(
                    v.static_data.max_rpm as f64,
                ),
                pit_limiter_engaged: v.physics.pit_limiter_on != 0,
                in_pit_lane: v.graphics.is_in_pit_lane != 0,
            },
            MomentSource::AssettoCorsaCompetizione(v) => BasicTelemetry {
                gear: (v.physics.gear - 1) as i8,
                speed: Velocity::new::<kilometer_per_hour>(v.physics.speed_kmh as f64),
                engine_rotation_speed: AngularVelocity::new::<revolution_per_minute>(
                    v.physics.rpm as f64,
                ),
                max_engine_rotation_speed: AngularVelocity::new::<revolution_per_minute>(
                    v.static_data.max_rpm as f64,
                ),
                pit_limiter_engaged: v.physics.pit_limiter_on,
                in_pit_lane: v.graphics.is_in_pit_lane,
            },
            MomentSource::RFactor2 {
                sim_state,
                player_id,
            } => {
                let player_id = player_id.as_ref()?;
                let player_telemetry = sim_state
                    .telemetry
                    .vehicles
                    .iter()
                    .find(|v| v.id == *player_id)?;
                let player_scoring = sim_state
                    .scoring
                    .vehicles
                    .iter()
                    .find(|v| v.id == *player_id)?;
                let speed_vec_ms = &player_telemetry.local_vel;
                let speed_ms = (speed_vec_ms.x * speed_vec_ms.x
                    + speed_vec_ms.y * speed_vec_ms.y
                    + speed_vec_ms.z * speed_vec_ms.z)
                    .sqrt();
                BasicTelemetry {
                    gear: player_telemetry.gear as i8,
                    speed: Velocity::new::<meter_per_second>(speed_ms),
                    engine_rotation_speed: AngularVelocity::new::<revolution_per_minute>(
                        player_telemetry.engine_rpm,
                    ),
                    max_engine_rotation_speed: AngularVelocity::new::<revolution_per_minute>(
                        player_telemetry.engine_max_rpm,
                    ),
                    pit_limiter_engaged: player_telemetry.speed_limiter != 0,
                    in_pit_lane: player_scoring.in_pits != 0,
                }
            }
        })
    }

    pub fn shift_point(&self) -> Option<AngularVelocity> {
        match &self.inner {
            MomentSource::IRacing(v) => Some(AngularVelocity::new::<revolution_per_minute>(
                v.session_info()["DriverInfo"]["DriverCarSLShiftRPM"].as_f64()?,
            )),
            MomentSource::AssettoCorsa(_)
            | MomentSource::AssettoCorsaCompetizione(_)
            | MomentSource::RFactor2 { .. } => None,
        }
    }

    pub fn flags(&self) {
        todo!();
    }
}
