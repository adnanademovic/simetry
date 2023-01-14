pub use crate::assetto_corsa::data::{
    FlagType, Graphics, Penalty, Physics, SessionType, StaticData, Status,
};
use crate::assetto_corsa::shared_memory_data::{PageFileGraphics, PageFilePhysics, PageFileStatic};
use crate::windows_util::SharedMemory;
use anyhow::Result;
use std::sync::Arc;
use std::time::Duration;

mod conversions;
mod data;
mod shared_memory_data;

pub struct SharedMemoryClient {
    static_data: Arc<StaticData>,
    physics_data: SharedMemory,
    graphics_data: SharedMemory,
    last_physics: Arc<Physics>,
    last_graphics: Arc<Graphics>,
}

#[derive(Clone, Debug)]
pub struct SimState {
    pub static_data: Arc<StaticData>,
    pub physics: Arc<Physics>,
    pub graphics: Arc<Graphics>,
}

impl SharedMemoryClient {
    pub async fn connect() -> Result<Self> {
        let poll_delay = Duration::from_millis(250);
        let graphics_data = SharedMemory::connect(b"Local\\acpmf_graphics\0", poll_delay).await;
        while !Self::is_connected(&graphics_data) {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }

        let physics_data = SharedMemory::connect(b"Local\\acpmf_physics\0", poll_delay).await;
        let static_data = Arc::new(
            unsafe {
                SharedMemory::connect(b"Local\\acpmf_static\0", poll_delay)
                    .await
                    .get_as::<PageFileStatic>()
            }
            .clone()
            .into(),
        );
        let last_physics = Arc::new(Self::physics(&physics_data));
        let last_graphics = Arc::new(Self::graphics(&physics_data));
        Ok(Self {
            static_data,
            physics_data,
            graphics_data,
            last_physics,
            last_graphics,
        })
    }

    pub async fn next_sim_state(&mut self) -> Option<SimState> {
        loop {
            if !Self::is_connected(&self.graphics_data) {
                return None;
            }
            let mut changed = false;
            let physics_packet_id =
                unsafe { self.physics_data.get_as::<PageFilePhysics>().packet_id };
            if self.last_physics.packet_id != physics_packet_id {
                changed = true;
                self.last_physics = Arc::new(Self::physics(&self.physics_data));
            }
            let graphics_packet_id =
                unsafe { self.graphics_data.get_as::<PageFilePhysics>().packet_id };
            if self.last_graphics.packet_id != graphics_packet_id {
                changed = true;
                self.last_graphics = Arc::new(Self::graphics(&self.graphics_data));
            }
            if changed {
                return Some(SimState {
                    static_data: Arc::clone(&self.static_data),
                    physics: Arc::clone(&self.last_physics),
                    graphics: Arc::clone(&self.last_graphics),
                });
            } else {
                tokio::time::sleep(Duration::from_millis(2)).await;
            }
        }
    }

    pub fn static_data(&self) -> &StaticData {
        &self.static_data
    }

    fn is_connected(graphics_data: &SharedMemory) -> bool {
        let status: Status = unsafe { graphics_data.get_as::<PageFileGraphics>() }
            .status
            .into();
        status != Status::Off
    }

    fn physics(physics_data: &SharedMemory) -> Physics {
        loop {
            let packet_id_1 = unsafe { physics_data.get_as::<PageFilePhysics>().packet_id };
            let data = unsafe { physics_data.get_as::<PageFilePhysics>() }.clone();
            let packet_id_2 = unsafe { physics_data.get_as::<PageFilePhysics>().packet_id };
            if packet_id_1 == packet_id_2 {
                return data.into();
            }
        }
    }

    fn graphics(graphics_data: &SharedMemory) -> Graphics {
        loop {
            let packet_id_1 = unsafe { graphics_data.get_as::<PageFileGraphics>().packet_id };
            let data = unsafe { graphics_data.get_as::<PageFileGraphics>() }.clone();
            let packet_id_2 = unsafe { graphics_data.get_as::<PageFileGraphics>().packet_id };
            if packet_id_1 == packet_id_2 {
                return data.into();
            }
        }
    }
}
