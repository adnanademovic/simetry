use crate::assetto_corsa_competizione::shared_memory_data::{
    PageFileGraphics, PageFilePhysics, PageFileStatic,
};
use crate::shared_memory_utilities::SharedMemTyped;
pub use anyhow::Result;
pub use data::{
    Aids, CarDamage, FlagType, GlobalFlags, Graphics, LapTiming, MfdPitstop, Penalty, Physics,
    RainIntensity, SessionType, StaticData, Status, Time, TrackGripStatus, Vector3, WheelInfo,
    Wheels,
};

mod constants;
mod conversions;
mod data;
mod shared_memory_data;

pub struct SharedMemoryClient {
    static_data: SharedMemTyped<PageFileStatic>,
    physics_data: SharedMemTyped<PageFilePhysics>,
    graphics_data: SharedMemTyped<PageFileGraphics>,
}

impl SharedMemoryClient {
    pub fn open() -> Result<Self> {
        Ok(Self {
            static_data: SharedMemTyped::open("Local\\acpmf_static")?,
            physics_data: SharedMemTyped::open("Local\\acpmf_physics")?,
            graphics_data: SharedMemTyped::open("Local\\acpmf_graphics")?,
        })
    }

    pub fn static_data(&self) -> StaticData {
        unsafe { self.static_data.get_raw() }.clone().into()
    }

    pub fn physics(&self) -> Physics {
        loop {
            let packet_id_1 = unsafe { self.physics_data.get_raw().packet_id };
            let data = unsafe { self.physics_data.get_raw() }.clone();
            let packet_id_2 = unsafe { self.physics_data.get_raw().packet_id };
            if packet_id_1 == packet_id_2 {
                return data.into();
            }
        }
    }

    pub fn graphics(&self) -> Graphics {
        loop {
            let packet_id_1 = unsafe { self.graphics_data.get_raw().packet_id };
            let data = unsafe { self.graphics_data.get_raw() }.clone();
            let packet_id_2 = unsafe { self.graphics_data.get_raw().packet_id };
            if packet_id_1 == packet_id_2 {
                return data.into();
            }
        }
    }
}
