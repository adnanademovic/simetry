use crate::assetto_corsa::util;
pub use crate::assetto_corsa_competizione::data::{
    Aids, CarDamage, FlagType, GlobalFlags, Graphics, LapTiming, MfdPitstop, Penalty, Physics,
    RainIntensity, SessionType, StaticData, Status, Time, TrackGripStatus, Vector3, WheelInfo,
    Wheels,
};
use crate::assetto_corsa_competizione::shared_memory_data::{
    PageFileGraphics, PageFilePhysics, PageFileStatic,
};
use crate::{BasicTelemetry, Moment, RacingFlags, Simetry};
use std::borrow::Cow;
use uom::si::angular_velocity::revolution_per_minute;
use uom::si::f64::{AngularVelocity, Velocity};
use uom::si::velocity::kilometer_per_hour;

mod conversions;
mod data;
mod shared_memory_data;

#[derive(Clone, Debug, Default)]
pub struct AssettoCorsaCompetizioneApiVersion;

impl util::AcApiVersion for AssettoCorsaCompetizioneApiVersion {
    const MAJOR_MIN: u16 = 1;
    const MAJOR_MAX: u16 = 1;
    const MINOR_MIN: u16 = 8;
    const MINOR_MAX: u16 = u16::MAX;
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

pub type Client = util::SharedMemoryClient<AssettoCorsaCompetizioneApiVersion>;
pub type SimState = util::SimState<AssettoCorsaCompetizioneApiVersion>;

#[async_trait::async_trait]
impl Simetry for Client {
    fn name(&self) -> &str {
        "AssettoCorsaCompetizione"
    }

    async fn next_moment(&mut self) -> Option<Box<dyn Moment>> {
        Some(Box::new(self.next_sim_state().await?))
    }
}

impl Moment for SimState {
    fn basic_telemetry(&self) -> Option<BasicTelemetry> {
        Some(BasicTelemetry {
            gear: (self.physics.gear - 1) as i8,
            speed: Velocity::new::<kilometer_per_hour>(self.physics.speed_kmh as f64),
            engine_rotation_speed: AngularVelocity::new::<revolution_per_minute>(
                self.physics.rpm as f64,
            ),
            max_engine_rotation_speed: AngularVelocity::new::<revolution_per_minute>(
                self.static_data.max_rpm as f64,
            ),
            pit_limiter_engaged: self.physics.pit_limiter_on,
            in_pit_lane: self.graphics.is_in_pit_lane,
        })
    }

    fn flags(&self) -> RacingFlags {
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
        flags
    }

    fn vehicle_unique_id(&self) -> Option<Cow<str>> {
        Some(self.static_data.car_model.as_str().into())
    }

    fn is_left_turn_indicator_on(&self) -> bool {
        self.graphics.direction_lights_left
    }

    fn is_right_turn_indicator_on(&self) -> bool {
        self.graphics.direction_lights_right
    }

    fn is_ignition_on(&self) -> bool {
        self.physics.ignition_on
    }

    fn is_starter_on(&self) -> bool {
        self.physics.starter_engine_on
    }
}
