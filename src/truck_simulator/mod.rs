use crate::windows_util::SharedMemory;
use crate::{Moment, Pedals, Simetry};
use anyhow::{bail, Result};
use std::borrow::Cow;
use std::time::Duration;
use uom::si::angular_velocity::revolution_per_minute;
use uom::si::f64::{AngularVelocity, Velocity};
use uom::si::velocity::kilometer_per_hour;

pub mod bindings;
pub mod json_client;

pub struct Client {
    shared_memory: SharedMemory,
    last_simulated_time: u64,
    game: Game,
}

impl Client {
    pub async fn connect(retry_delay: Duration) -> Self {
        loop {
            if let Ok(v) = Self::try_connect().await {
                return v;
            }
            tokio::time::sleep(retry_delay).await
        }
    }

    pub async fn try_connect() -> Result<Self> {
        let poll_delay = Duration::from_millis(250);
        let shared_memory = SharedMemory::connect(b"Local\\SCSTelemetry\0", poll_delay).await;
        let sim_state = Self::inner_next_sim_state(&shared_memory)?;
        if !sim_state.shared.sdkActive {
            bail!("SDK is not active");
        }
        Ok(Self {
            shared_memory,
            last_simulated_time: sim_state.shared.simulatedTime,
            game: sim_state.game,
        })
    }

    fn inner_next_sim_state(shared_memory: &SharedMemory) -> Result<SimState> {
        loop {
            let shared = unsafe { shared_memory.copy_as::<bindings::scsTelemetryMap_t>() };
            let shared_retry = unsafe { shared_memory.copy_as::<bindings::scsTelemetryMap_t>() };
            if shared != shared_retry {
                // Retry until we are sure we didn't catch shared memory mid-write
                continue;
            }
            if shared.scs_values.telemetry_plugin_revision != bindings::PLUGIN_REVID {
                bail!(
                    "Plugin revision {} is incompatible with {} version from the DLL",
                    bindings::PLUGIN_REVID,
                    shared.scs_values.telemetry_plugin_revision,
                );
            }
            let game = Game::from_id(shared.scs_values.game)?;
            return Ok(SimState { shared, game });
        }
    }

    pub async fn next_sim_state(&mut self) -> Result<SimState> {
        loop {
            let sim_state = Self::inner_next_sim_state(&self.shared_memory)?;
            if sim_state.shared.simulatedTime == self.last_simulated_time {
                // Querying too frequently
                continue;
            }
            self.last_simulated_time = sim_state.shared.simulatedTime;
            return Ok(sim_state);
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Game {
    Ets2,
    Ats,
}

impl Game {
    fn from_id(id: u32) -> Result<Self> {
        Ok(match id {
            bindings::ETS2 => Game::Ets2,
            bindings::ATS => Game::Ats,
            _ => bail!("SCS game with ID {id} not supported"),
        })
    }
}

#[derive(Debug)]
pub struct SimState {
    pub game: Game,
    pub shared: bindings::scsTelemetryMap_t,
}

impl SimState {
    pub fn parse_string(data: &[i8]) -> String {
        String::from_utf8(
            data.iter()
                .map_while(|v| if *v < 32 { None } else { Some(*v as u8) })
                .collect(),
        )
        .unwrap()
    }
}

#[async_trait::async_trait]
impl Simetry for Client {
    fn name(&self) -> &str {
        match self.game {
            Game::Ets2 => "ETS2",
            Game::Ats => "ATS",
        }
    }

    async fn next_moment(&mut self) -> Option<Box<dyn Moment + Send + Sync + 'static>> {
        Some(Box::new(self.next_sim_state().await.ok()?))
    }
}

impl Moment for SimState {
    fn vehicle_gear(&self) -> Option<i8> {
        Some(self.shared.truck_i.gear as i8)
    }

    fn vehicle_velocity(&self) -> Option<Velocity> {
        Some(Velocity::new::<kilometer_per_hour>(
            self.shared.truck_f.speed as f64,
        ))
    }

    fn vehicle_engine_rotation_speed(&self) -> Option<AngularVelocity> {
        Some(AngularVelocity::new::<revolution_per_minute>(
            self.shared.truck_f.engineRpm as f64,
        ))
    }

    fn vehicle_max_engine_rotation_speed(&self) -> Option<AngularVelocity> {
        Some(AngularVelocity::new::<revolution_per_minute>(
            self.shared.config_f.engineRpmMax as f64,
        ))
    }

    fn vehicle_brand_id(&self) -> Option<Cow<str>> {
        Some(Self::parse_string(&self.shared.config_s.truckBrand).into())
    }

    fn vehicle_model_id(&self) -> Option<Cow<str>> {
        Some(Self::parse_string(&self.shared.config_s.truckName).into())
    }

    fn vehicle_unique_id(&self) -> Option<Cow<str>> {
        Some(Self::parse_string(&self.shared.config_s.truckId).into())
    }

    fn is_left_turn_indicator_on(&self) -> Option<bool> {
        Some(self.shared.truck_b.blinkerLeftActive)
    }

    fn is_right_turn_indicator_on(&self) -> Option<bool> {
        Some(self.shared.truck_b.blinkerRightActive)
    }

    fn is_hazard_indicator_on(&self) -> Option<bool> {
        Some(self.shared.truck_b.lightsHazard)
    }

    fn is_ignition_on(&self) -> Option<bool> {
        Some(self.shared.truck_b.electricEnabled)
    }

    fn pedals(&self) -> Option<Pedals> {
        Some(Pedals {
            throttle: self.shared.truck_f.gameThrottle as f64,
            brake: self.shared.truck_f.gameBrake as f64,
            clutch: self.shared.truck_f.gameClutch as f64,
        })
    }

    fn pedals_raw(&self) -> Option<Pedals> {
        Some(Pedals {
            throttle: self.shared.truck_f.userThrottle as f64,
            brake: self.shared.truck_f.userBrake as f64,
            clutch: self.shared.truck_f.userClutch as f64,
        })
    }
}
