use crate::assetto_corsa_competizione::shared_memory_data::{
    PageFileGraphics, PageFilePhysics, PageFileStatic,
};
use crate::windows_util::SharedMemory;
pub use anyhow::Result;
pub use data::{
    Aids, CarDamage, FlagType, GlobalFlags, Graphics, LapTiming, MfdPitstop, Penalty, Physics,
    RainIntensity, SessionType, StaticData, Status, Time, TrackGripStatus, Vector3, WheelInfo,
    Wheels,
};
use std::time::Duration;

mod constants;
mod conversions;
mod data;
mod shared_memory_data;

pub struct SharedMemoryClient {
    static_data: SharedMemory,
    physics_data: SharedMemory,
    graphics_data: SharedMemory,
}

impl SharedMemoryClient {
    pub async fn connect() -> Result<Self> {
        let poll_delay = Duration::from_millis(250);
        let static_data = SharedMemory::connect(b"Local\\acpmf_static\0", poll_delay);
        let physics_data = SharedMemory::connect(b"Local\\acpmf_physics\0", poll_delay);
        let graphics_data = SharedMemory::connect(b"Local\\acpmf_graphics\0", poll_delay);
        Ok(Self {
            static_data: static_data.await,
            physics_data: physics_data.await,
            graphics_data: graphics_data.await,
        })
    }

    pub fn static_data(&self) -> StaticData {
        unsafe { self.static_data.get_as::<PageFileStatic>() }
            .clone()
            .into()
    }

    pub fn physics(&self) -> Physics {
        loop {
            let packet_id_1 = unsafe { self.physics_data.get_as::<PageFilePhysics>().packet_id };
            let data = unsafe { self.physics_data.get_as::<PageFilePhysics>() }.clone();
            let packet_id_2 = unsafe { self.physics_data.get_as::<PageFilePhysics>().packet_id };
            if packet_id_1 == packet_id_2 {
                return data.into();
            }
        }
    }

    pub fn graphics(&self) -> Graphics {
        loop {
            let packet_id_1 = unsafe { self.graphics_data.get_as::<PageFileGraphics>().packet_id };
            let data = unsafe { self.graphics_data.get_as::<PageFileGraphics>() }.clone();
            let packet_id_2 = unsafe { self.graphics_data.get_as::<PageFileGraphics>().packet_id };
            if packet_id_1 == packet_id_2 {
                return data.into();
            }
        }
    }
}
