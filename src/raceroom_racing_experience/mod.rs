use crate::windows_util::SharedMemory;
use crate::{Moment, Simetry};
use anyhow::{bail, Result};
use std::time::Duration;

pub mod bindings;

pub struct Client {
    shared_memory: SharedMemory,
}

impl Client {
    pub async fn connect(retry_delay: Duration) -> Self {
        loop {
            if let Ok(v) = Self::try_connect().await {
                return v;
            }
            tokio::time::sleep(retry_delay).await
        }
    }

    pub async fn try_connect() -> Result<Self> {
        let poll_delay = Duration::from_millis(250);
        let shared_memory =
            SharedMemory::connect(bindings::R3E_SHARED_MEMORY_NAME, poll_delay).await;
        Ok(Self { shared_memory })
    }

    pub async fn next_sim_state(&mut self) -> Result<SimState> {
        loop {
            let r3e_shared = unsafe { self.shared_memory.copy_as::<bindings::r3e_shared>() };
            let r3e_shared_retry = unsafe { self.shared_memory.copy_as::<bindings::r3e_shared>() };
            if r3e_shared != r3e_shared_retry {
                // Retry until we are sure we didn't catch shared memory mid-write
                continue;
            }
            if r3e_shared.version_major != bindings::R3E_VERSION_MAJOR
                || r3e_shared.version_minor < bindings::R3E_VERSION_MINOR
            {
                let major = r3e_shared.version_major;
                let minor = r3e_shared.version_minor;
                bail!(
                    "API version {}.{} is incompatible with {}.{} version from the game",
                    bindings::R3E_VERSION_MAJOR,
                    bindings::R3E_VERSION_MINOR,
                    major,
                    minor,
                );
            }
            return Ok(SimState { r3e_shared });
        }
    }
}

#[derive(Debug)]
pub struct SimState {
    pub r3e_shared: bindings::r3e_shared,
}

#[async_trait::async_trait]
impl Simetry for Client {
    fn name(&self) -> &str {
        "RaceRoomRacingExperience"
    }

    async fn next_moment(&mut self) -> Option<Box<dyn Moment>> {
        Some(Box::new(self.next_sim_state().await.ok()?))
    }
}

impl Moment for SimState {}
