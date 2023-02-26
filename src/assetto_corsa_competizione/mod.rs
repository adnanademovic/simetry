use crate::assetto_corsa::util;
pub use crate::assetto_corsa_competizione::data::{
    Aids, CarDamage, FlagType, GlobalFlags, Graphics, LapTiming, MfdPitstop, Penalty, Physics,
    RainIntensity, SessionType, StaticData, Status, Time, TrackGripStatus, Vector3, WheelInfo,
    Wheels,
};
use crate::assetto_corsa_competizione::shared_memory_data::{
    PageFileGraphics, PageFilePhysics, PageFileStatic,
};
use crate::{BasicTelemetry, MomentImpl, RacingFlags};
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

impl MomentImpl for SimState {
    fn car_left(&self) -> bool {
        false
    }

    fn car_right(&self) -> bool {
        false
    }

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

    fn shift_point(&self) -> Option<AngularVelocity> {
        None
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

    fn car_model_id(&self) -> Option<String> {
        Some(self.static_data.car_model.clone())
    }
}
