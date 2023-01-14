pub use crate::assetto_corsa::data::{
    FlagType, Graphics, Penalty, Physics, SessionType, StaticData, Status,
};
use crate::assetto_corsa::shared_memory_data::{PageFileGraphics, PageFilePhysics, PageFileStatic};

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

pub type SharedMemoryClient = util::SharedMemoryClient<AssettoCorsaApiVersion>;
pub type SimState = util::SimState<AssettoCorsaApiVersion>;
