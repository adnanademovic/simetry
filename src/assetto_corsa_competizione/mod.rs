use crate::assetto_corsa::util;
pub use crate::assetto_corsa_competizione::data::{
    Aids, CarDamage, FlagType, GlobalFlags, Graphics, LapTiming, MfdPitstop, Penalty, Physics,
    RainIntensity, SessionType, StaticData, Status, Time, TrackGripStatus, Vector3, WheelInfo,
    Wheels,
};
use crate::assetto_corsa_competizione::shared_memory_data::{
    PageFileGraphics, PageFilePhysics, PageFileStatic,
};

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
