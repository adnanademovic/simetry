pub use crate::assetto_corsa::data::{
    FlagType, Graphics, Penalty, Physics, SessionType, StaticData, Status,
};
use crate::assetto_corsa::shared_memory_data::{PageFileGraphics, PageFilePhysics, PageFileStatic};
use crate::{Moment, RacingFlags, Simetry};
use std::borrow::Cow;
use uom::si::angular_velocity::revolution_per_minute;
use uom::si::f64::{AngularVelocity, Velocity};
use uom::si::velocity::kilometer_per_hour;

mod conversions;
mod data;
mod shared_memory_data;
pub(crate) mod util;

#[derive(Clone, Debug, Default)]
pub struct AssettoCorsaApiVersion;

impl util::AcApiVersion for AssettoCorsaApiVersion {
    const MAJOR_MIN: u16 = 1;
    const MAJOR_MAX: u16 = 1;
    const MINOR_MIN: u16 = 0;
    const MINOR_MAX: u16 = 7;
    type PageStatic = PageFileStatic;
    type DataStatic = StaticData;
    type PagePhysics = PageFilePhysics;
    type DataPhysics = Physics;
    type PageGraphics = PageFileGraphics;
    type DataGraphics = Graphics;
}

impl util::WithPacketId for PageFilePhysics {
    fn packet_id(&self) -> i32 {
        self.packet_id
    }
}

impl util::WithPacketId for Physics {
    fn packet_id(&self) -> i32 {
        self.packet_id
    }
}

impl util::WithPacketId for PageFileGraphics {
    fn packet_id(&self) -> i32 {
        self.packet_id
    }
}

impl util::WithPacketId for Graphics {
    fn packet_id(&self) -> i32 {
        self.packet_id
    }
}

pub type Client = util::SharedMemoryClient<AssettoCorsaApiVersion>;
pub type SimState = util::SimState<AssettoCorsaApiVersion>;

#[async_trait::async_trait]
impl Simetry for Client {
    fn name(&self) -> &str {
        "AssettoCorsa"
    }

    async fn next_moment(&mut self) -> Option<Box<dyn Moment + Send + Sync + 'static>> {
        Some(Box::new(self.next_sim_state().await?))
    }
}

impl Moment for SimState {
    fn vehicle_gear(&self) -> Option<i8> {
        Some((self.physics.gear - 1) as i8)
    }

    fn vehicle_velocity(&self) -> Option<Velocity> {
        Some(Velocity::new::<kilometer_per_hour>(
            self.physics.speed_kmh as f64,
        ))
    }

    fn vehicle_engine_rotation_speed(&self) -> Option<AngularVelocity> {
        Some(AngularVelocity::new::<revolution_per_minute>(
            self.physics.rpm as f64,
        ))
    }

    fn vehicle_max_engine_rotation_speed(&self) -> Option<AngularVelocity> {
        Some(AngularVelocity::new::<revolution_per_minute>(
            self.static_data.max_rpm as f64,
        ))
    }

    fn is_pit_limiter_engaged(&self) -> Option<bool> {
        Some(self.physics.pit_limiter_on != 0)
    }

    fn is_vehicle_in_pit_lane(&self) -> Option<bool> {
        Some(self.graphics.is_in_pit_lane != 0)
    }

    fn flags(&self) -> Option<RacingFlags> {
        let mut flags = RacingFlags::default();
        match self.graphics.flag {
            FlagType::None => {}
            FlagType::Blue => flags.blue = true,
            FlagType::Yellow => flags.yellow = true,
            FlagType::Black => flags.black = true,
            FlagType::White => flags.white = true,
            FlagType::Checkered => flags.checkered = true,
            FlagType::Penalty => flags.black = true,
            FlagType::Green => flags.green = true,
            FlagType::Orange => flags.meatball = true,
        }
        Some(flags)
    }

    fn vehicle_unique_id(&self) -> Option<Cow<str>> {
        Some(self.static_data.car_model.as_str().into())
    }

    fn is_left_turn_indicator_on(&self) -> Option<bool> {
        Some(self.graphics.direction_lights_left != 0)
    }

    fn is_right_turn_indicator_on(&self) -> Option<bool> {
        Some(self.graphics.direction_lights_right != 0)
    }

    fn is_ignition_on(&self) -> Option<bool> {
        Some(self.physics.ignition_on != 0)
    }

    fn is_starter_on(&self) -> Option<bool> {
        Some(self.physics.starter_engine_on != 0)
    }
}
