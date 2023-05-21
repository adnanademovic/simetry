use crate::assetto_corsa::conversions::extract_string;
use crate::assetto_corsa::shared_memory_data::StatusRaw;
use crate::assetto_corsa::Status;
use crate::windows_util::SharedMemory;
use anyhow::{bail, Context, Result};
use std::fmt::Debug;
use std::sync::Arc;
use std::time::Duration;

#[repr(C, packed(4))]
#[derive(Clone, Debug)]
pub struct PageFileGraphicsTop {
    pub packet_id: i32,
    pub status: StatusRaw,
}

#[repr(C, packed(4))]
#[derive(Clone, Debug)]
pub struct PageFileStaticTop {
    pub sm_version: [u16; 15],
}

pub struct SharedMemoryClient<Version: AcApiVersion> {
    static_data: Arc<Version::DataStatic>,
    physics_data: SharedMemory,
    graphics_data: SharedMemory,
    last_physics: Arc<Version::DataPhysics>,
    last_graphics: Arc<Version::DataGraphics>,
}

pub trait WithPacketId {
    fn packet_id(&self) -> i32;
}

pub trait AcApiVersion: Debug {
    const MAJOR_MIN: u16;
    const MAJOR_MAX: u16;
    const MINOR_MIN: u16;
    const MINOR_MAX: u16;
    type PageStatic: Clone + Into<Self::DataStatic>;
    type DataStatic: Clone + Debug;
    type PagePhysics: Clone + WithPacketId + Into<Self::DataPhysics>;
    type DataPhysics: Clone + WithPacketId + Debug;
    type PageGraphics: Clone + WithPacketId + Into<Self::DataGraphics>;
    type DataGraphics: Clone + WithPacketId + Debug;
}

#[derive(Clone, Debug)]
pub struct SimState<Version: AcApiVersion> {
    pub static_data: Arc<Version::DataStatic>,
    pub physics: Arc<Version::DataPhysics>,
    pub graphics: Arc<Version::DataGraphics>,
}

impl<Version: AcApiVersion> SharedMemoryClient<Version> {
    pub async fn connect(retry_delay: Duration) -> Self {
        loop {
            if let Ok(v) = Self::try_connect().await {
                return v;
            }
            tokio::time::sleep(retry_delay).await;
        }
    }

    pub async fn try_connect() -> Result<Self> {
        let poll_delay = Duration::from_millis(250);
        let graphics_data = SharedMemory::connect(b"Local\\acpmf_graphics\0", poll_delay).await;
        while !Self::is_connected(&graphics_data) {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        let physics_data = SharedMemory::connect(b"Local\\acpmf_physics\0", poll_delay).await;
        let static_data = SharedMemory::connect(b"Local\\acpmf_static\0", poll_delay).await;

        Self::check_version(&static_data)?;
        let static_data = Arc::new(
            unsafe { static_data.get_as::<Version::PageStatic>() }
                .clone()
                .into(),
        );
        let last_physics = Arc::new(Self::physics(&physics_data));
        let last_graphics = Arc::new(Self::graphics(&graphics_data));
        Ok(Self {
            static_data,
            physics_data,
            graphics_data,
            last_physics,
            last_graphics,
        })
    }

    fn is_connected(graphics_data: &SharedMemory) -> bool {
        let status: Status = unsafe { graphics_data.get_as::<PageFileGraphicsTop>() }
            .status
            .into();
        status != Status::Off
    }

    fn check_version(static_memory: &SharedMemory) -> Result<()> {
        let sm_version = unsafe { static_memory.get_as::<PageFileStaticTop>() }
            .clone()
            .sm_version;
        let sm_version_string = extract_string(&sm_version);
        let sm_version = sm_version_string
            .split('.')
            .map(|v| v.parse::<u16>())
            .collect::<Result<Vec<u16>, _>>()
            .with_context(|| {
                format!("Invalid shared memory version string: {sm_version_string:?}")
            })?;
        let Some(major) = sm_version.first().copied() else {
            bail!("Shared memory version is missing major version in: {sm_version_string:?}");
        };
        let Some(minor) = sm_version.get(1).copied() else {
            bail!("Shared memory version is missing minor version in: {sm_version_string:?}");
        };
        let min_version = ((Version::MAJOR_MIN as u32) << 16) | (Version::MINOR_MIN as u32);
        let max_version = ((Version::MAJOR_MAX as u32) << 16) | (Version::MINOR_MAX as u32);
        let version = ((major as u32) << 16) | (minor as u32);
        if version < min_version || version > max_version {
            bail!(
                "Expected shared memory major version in {}.{} - {}.{} range, got {}.{}",
                Version::MAJOR_MIN,
                Version::MINOR_MIN,
                Version::MAJOR_MAX,
                Version::MINOR_MAX,
                major,
                minor,
            );
        }
        Ok(())
    }

    pub async fn next_sim_state(&mut self) -> Option<SimState<Version>> {
        loop {
            if !Self::is_connected(&self.graphics_data) {
                return None;
            }
            let mut changed = false;
            let physics_packet_id = unsafe {
                self.physics_data
                    .get_as::<Version::PagePhysics>()
                    .packet_id()
            };
            if self.last_physics.packet_id() != physics_packet_id {
                changed = true;
                self.last_physics = Arc::new(Self::physics(&self.physics_data));
            }
            let graphics_packet_id = unsafe {
                self.graphics_data
                    .get_as::<Version::PageGraphics>()
                    .packet_id()
            };
            if self.last_graphics.packet_id() != graphics_packet_id {
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

    pub fn static_data(&self) -> &Version::DataStatic {
        &self.static_data
    }

    fn physics(physics_data: &SharedMemory) -> Version::DataPhysics {
        loop {
            let packet_id_1 = unsafe { physics_data.get_as::<Version::PagePhysics>().packet_id() };
            let data = unsafe { physics_data.get_as::<Version::PagePhysics>() }.clone();
            let packet_id_2 = unsafe { physics_data.get_as::<Version::PagePhysics>().packet_id() };
            if packet_id_1 == packet_id_2 {
                return data.into();
            }
        }
    }

    fn graphics(graphics_data: &SharedMemory) -> Version::DataGraphics {
        loop {
            let packet_id_1 =
                unsafe { graphics_data.get_as::<Version::PageGraphics>().packet_id() };
            let data = unsafe { graphics_data.get_as::<Version::PageGraphics>() }.clone();
            let packet_id_2 =
                unsafe { graphics_data.get_as::<Version::PageGraphics>().packet_id() };
            if packet_id_1 == packet_id_2 {
                return data.into();
            }
        }
    }
}
