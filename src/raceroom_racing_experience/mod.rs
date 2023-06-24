use crate::windows_util::SharedMemory;
use crate::{Moment, RacingFlags, Simetry};
use anyhow::{bail, Result};
use std::borrow::Cow;
use std::time::Duration;
use uom::si::angular_velocity::radian_per_second;
use uom::si::f64::{AngularVelocity, Velocity};
use uom::si::velocity::meter_per_second;

pub mod bindings;

pub struct Client {
    shared_memory: SharedMemory,
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
        let shared_memory =
            SharedMemory::connect(bindings::R3E_SHARED_MEMORY_NAME, poll_delay).await;
        Ok(Self { shared_memory })
    }

    pub async fn next_sim_state(&mut self) -> Result<SimState> {
        loop {
            let r3e_shared = unsafe { self.shared_memory.copy_as::<bindings::r3e_shared>() };
            let r3e_shared_retry = unsafe { self.shared_memory.copy_as::<bindings::r3e_shared>() };
            if r3e_shared != r3e_shared_retry {
                // Retry until we are sure we didn't catch shared memory mid-write
                continue;
            }
            if r3e_shared.version_major != bindings::R3E_VERSION_MAJOR
                || r3e_shared.version_minor < bindings::R3E_VERSION_MINOR
            {
                let major = r3e_shared.version_major;
                let minor = r3e_shared.version_minor;
                bail!(
                    "API version {}.{} is incompatible with {}.{} version from the game",
                    bindings::R3E_VERSION_MAJOR,
                    bindings::R3E_VERSION_MINOR,
                    major,
                    minor,
                );
            }
            return Ok(SimState { r3e_shared });
        }
    }
}

#[derive(Debug)]
pub struct SimState {
    pub r3e_shared: bindings::r3e_shared,
}

#[async_trait::async_trait]
impl Simetry for Client {
    fn name(&self) -> &str {
        "RaceRoomRacingExperience"
    }

    async fn next_moment(&mut self) -> Option<Box<dyn Moment>> {
        Some(Box::new(self.next_sim_state().await.ok()?))
    }
}

impl Moment for SimState {
    fn vehicle_gear(&self) -> Option<i8> {
        let gear = self.r3e_shared.gear as i8;
        if gear == -2 {
            return None;
        }
        Some(gear)
    }

    fn vehicle_velocity(&self) -> Option<Velocity> {
        Some(Velocity::new::<meter_per_second>(
            self.r3e_shared.car_speed as f64,
        ))
    }

    fn vehicle_engine_rotation_speed(&self) -> Option<AngularVelocity> {
        Some(AngularVelocity::new::<radian_per_second>(
            self.r3e_shared.engine_rps as f64,
        ))
    }

    fn vehicle_max_engine_rotation_speed(&self) -> Option<AngularVelocity> {
        Some(AngularVelocity::new::<radian_per_second>(
            self.r3e_shared.max_engine_rps as f64,
        ))
    }

    fn is_pit_limiter_engaged(&self) -> Option<bool> {
        let pit_limiter = self.r3e_shared.pit_limiter;
        if pit_limiter == -1 {
            return None;
        }
        Some(pit_limiter != 0)
    }

    fn is_vehicle_in_pit_lane(&self) -> Option<bool> {
        let in_pitlane = self.r3e_shared.in_pitlane;
        if in_pitlane == -1 {
            return None;
        }
        Some(in_pitlane != 0)
    }

    fn shift_point(&self) -> Option<AngularVelocity> {
        Some(AngularVelocity::new::<radian_per_second>(
            self.r3e_shared.upshift_rps as f64,
        ))
    }

    fn flags(&self) -> Option<RacingFlags> {
        let flags = self.r3e_shared.flags;
        Some(RacingFlags {
            green: flags.green > 0,
            yellow: flags.yellow > 0,
            blue: flags.blue > 0,
            white: flags.white > 0,
            red: false,
            black: flags.black > 0,
            checkered: flags.checkered > 0,
            meatball: false,
            black_and_white: flags.black_and_white > 0,
            start_ready: false,
            start_set: false,
            start_go: false,
        })
    }

    fn vehicle_brand_id(&self) -> Option<Cow<str>> {
        let value = self.r3e_shared.vehicle_info.manufacturer_id;
        Some(value.to_string().into())
    }

    fn vehicle_model_id(&self) -> Option<Cow<str>> {
        let value = self.r3e_shared.vehicle_info.model_id;
        Some(value.to_string().into())
    }
}
