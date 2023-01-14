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
    static_data: StaticData,
    physics_data: SharedMemory,
    graphics_data: SharedMemory,
}

impl SharedMemoryClient {
    pub async fn connect() -> Result<Self> {
        let poll_delay = Duration::from_millis(250);
        let graphics_data = SharedMemory::connect(b"Local\\acpmf_graphics\0", poll_delay).await;
        loop {
            let status: Status = unsafe { graphics_data.get_as::<PageFileGraphics>() }
                .status
                .into();
            if status != Status::Off {
                break;
            }
            tokio::time::sleep(Duration::from_millis(5)).await;
        }

        let physics_data = SharedMemory::connect(b"Local\\acpmf_physics\0", poll_delay).await;
        let static_data = unsafe {
            SharedMemory::connect(b"Local\\acpmf_static\0", poll_delay)
                .await
                .get_as::<PageFileStatic>()
        }
        .clone()
        .into();
        Ok(Self {
            static_data,
            physics_data: physics_data,
            graphics_data: graphics_data,
        })
    }

    pub fn is_connected(&self) -> bool {
        let status: Status = unsafe { self.graphics_data.get_as::<PageFileGraphics>() }
            .status
            .into();
        status != Status::Off
    }

    pub fn static_data(&self) -> &StaticData {
        &self.static_data
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
