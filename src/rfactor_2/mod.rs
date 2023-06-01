//! Support for rFactor 2.
//!
//! Requires installing and enabling plugin from <https://github.com/TheIronWolfModding/rF2SharedMemoryMapPlugin>.

mod client;
mod data;
mod shared_memory_data;

use crate::{BasicTelemetry, Moment, RacingFlags, Simetry};
pub use client::{Client, Config};
pub use data::{Extended, ForceFeedback, MultiRules, PitInfo, Rules, Scoring, Telemetry, Weather};
use std::sync::Arc;
use uom::si::angular_velocity::revolution_per_minute;
use uom::si::f64::{AngularVelocity, Velocity};
use uom::si::velocity::meter_per_second;

#[derive(Clone, Debug)]
pub struct SimState {
    pub telemetry: Arc<Telemetry>,
    pub scoring: Arc<Scoring>,
    pub rules: Arc<Rules>,
    pub multi_rules: Arc<MultiRules>,
    pub force_feedback: Arc<ForceFeedback>,
    pub pit_info: Arc<PitInfo>,
    pub weather: Arc<Weather>,
    pub extended: Arc<Extended>,
}

#[async_trait::async_trait]
impl Simetry for Client {
    fn name(&self) -> &str {
        "rFactor2"
    }

    async fn next_moment(&mut self) -> Option<Box<dyn Moment>> {
        Some(Box::new(self.next_sim_state().await?))
    }
}

impl Moment for SimState {
    fn basic_telemetry(&self) -> Option<BasicTelemetry> {
        let player_scoring = self.scoring.vehicles.iter().find(|v| v.is_player != 0)?;
        let player_id = player_scoring.id;
        let player_telemetry = self.telemetry.vehicles.iter().find(|v| v.id == player_id)?;
        let speed_vec_ms = &player_telemetry.local_vel;
        let speed_ms = (speed_vec_ms.x * speed_vec_ms.x
            + speed_vec_ms.y * speed_vec_ms.y
            + speed_vec_ms.z * speed_vec_ms.z)
            .sqrt();
        Some(BasicTelemetry {
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
        })
    }

    fn flags(&self) -> RacingFlags {
        let player_scoring = match self.scoring.vehicles.iter().find(|v| v.is_player != 0) {
            Some(v) => v,
            None => return RacingFlags::default(),
        };
        RacingFlags {
            green: player_scoring.flag == 0,
            yellow: player_scoring.individual_phase == 10,
            blue: player_scoring.flag == 6,
            white: false,
            red: false,
            black: player_scoring.num_penalties > 0,
            checkered: player_scoring.finish_status == 1,
            meatball: false,
            black_and_white: false,
            start_ready: false,
            start_set: false,
            start_go: false,
        }
    }

    fn car_model_id(&self) -> Option<String> {
        let player_scoring = self.scoring.vehicles.iter().find(|v| v.is_player != 0)?;
        Some(
            player_scoring
                .vehicle_name
                .split_once('#')
                .unwrap_or(("?", ""))
                .0
                .trim()
                .into(),
        )
    }
}
