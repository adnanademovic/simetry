use crate::iracing::constants::IRSDK_VER;
use crate::iracing::header::{VarBuf, VarHeaderRaw};
use crate::iracing::session_info::parse_session_info;
use crate::iracing::{Header, SimState, VarHeader, VarHeaders};
use crate::{windows_util, Moment, Simetry};
use anyhow::{bail, Result};
use std::collections::HashMap;
use std::slice::from_raw_parts;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::task::spawn_blocking;
use windows::core::PCSTR;
use windows::Win32::System::Threading::{
    OpenEventA, WaitForSingleObject, SYNCHRONIZATION_SYNCHRONIZE,
};
use windows::Win32::System::WindowsProgramming::INFINITE;
use yaml_rust::Yaml;

static DATAVALIDEVENTNAME: &[u8] = b"Local\\IRSDKDataValidEvent\0";
static MEMMAPFILENAME: &[u8] = b"Local\\IRSDKMemMapFileName\0";

const STATUS_CONNECTED_FLAG: i32 = 1;

const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);

pub struct Client {
    vars_at_buf_len: i32,
    vars: Arc<VarHeaders>,
    session_info_cache: SessionInfoCache,
    last_tick_count: i32,
    last_valid_time: Option<SystemTime>,

    shared_memory: SharedMemory,
    data_valid_event: DataValidEvent,
}

impl Client {
    pub async fn connect(retry_delay: Duration) -> Self {
        loop {
            if let Ok(v) = Self::try_connect().await {
                return v;
            }
            tokio::time::sleep(retry_delay).await;
        }
    }

    pub async fn try_connect() -> Result<Self> {
        let shared_memory = SharedMemory::connect();
        let data_valid_event = DataValidEvent::connect();

        let shared_memory = shared_memory.await;
        let data_valid_event = data_valid_event.await;

        while !shared_memory.is_header_connected() {
            data_valid_event
                .wait(Some(Duration::from_millis(250)))
                .await;
        }

        let sdk_version = shared_memory.header().ver;
        if sdk_version != IRSDK_VER {
            bail!("iRacing SDK version mismatch: expected {IRSDK_VER}, received {sdk_version}");
        }

        Ok(Client {
            vars_at_buf_len: -1,
            vars: Arc::new(HashMap::new()),
            session_info_cache: SessionInfoCache::default(),
            last_tick_count: i32::MAX,
            last_valid_time: None,
            shared_memory,
            data_valid_event,
        })
    }

    pub async fn next_sim_state(&mut self) -> Option<SimState> {
        loop {
            if !self.is_connected() {
                return None;
            }

            if let Some(sim_state) = self.get_new_sim_state() {
                return Some(sim_state);
            }

            self.data_valid_event
                .wait(Some(Duration::from_millis(250)))
                .await;
        }
    }

    fn get_new_sim_state(&mut self) -> Option<SimState> {
        let header = self.shared_memory.header();

        if self.vars_at_buf_len != header.buf_len {
            self.vars = Arc::new(self.shared_memory.get_var_headers());
        }

        if header.status & STATUS_CONNECTED_FLAG == 0 {
            self.last_tick_count = i32::MAX;
            return None;
        }

        let mut latest_buffer_idx = 0;
        for idx in 1..(header.num_buf as usize) {
            if header.var_buf[latest_buffer_idx].tick_count < header.var_buf[idx].tick_count {
                latest_buffer_idx = idx;
            }
        }

        let buffer = &header.var_buf[latest_buffer_idx];

        if self.last_tick_count >= buffer.tick_count {
            self.last_tick_count = buffer.tick_count;
            return None;
        }

        // Two attempts to retrieve data
        for _ in 0..2 {
            let tick_count = buffer.tick_count;
            let data = self.shared_memory.data(header, buffer);
            if tick_count == buffer.tick_count {
                self.last_tick_count = tick_count;
                self.last_valid_time = Some(SystemTime::now());
                return if self.is_connected() {
                    let session_info = self.session_info_cache.get(&self.shared_memory).ok()?;
                    Some(SimState::new(
                        Arc::new(header.clone()),
                        Arc::clone(&self.vars),
                        data.to_vec(),
                        session_info,
                    ))
                } else {
                    None
                };
            }
        }

        None
    }

    fn is_connected(&self) -> bool {
        if !self.shared_memory.is_header_connected() {
            return false;
        }
        let last_valid_time = match self.last_valid_time {
            Some(v) => v,
            None => {
                return true;
            }
        };
        let elapsed = match SystemTime::now().duration_since(last_valid_time) {
            Ok(v) => v,
            Err(_) => return false,
        };
        elapsed < CLIENT_TIMEOUT
    }
}

#[async_trait::async_trait]
impl Simetry for Client {
    fn name(&self) -> &str {
        "iRacing"
    }

    async fn next_moment(&mut self) -> Option<Box<dyn Moment + Send + Sync + 'static>> {
        Some(Box::new(self.next_sim_state().await?))
    }
}

#[derive(Default)]
struct SessionInfoCache {
    content: Option<(i32, Arc<Yaml>)>,
}

impl SessionInfoCache {
    fn get(&mut self, shared_memory: &SharedMemory) -> Result<Arc<Yaml>> {
        let new_id = shared_memory.header().session_info_update;
        if let Some((old_id, data)) = &self.content {
            if new_id == *old_id {
                return Ok(Arc::clone(data));
            }
        }
        let session_info = Arc::new(parse_session_info(shared_memory.raw_session_info())?);
        self.content = Some((new_id, Arc::clone(&session_info)));
        Ok(session_info)
    }
}

struct SharedMemory(windows_util::SharedMemory);

impl SharedMemory {
    async fn connect() -> Self {
        Self(windows_util::SharedMemory::connect(MEMMAPFILENAME, Duration::from_millis(250)).await)
    }

    fn header(&self) -> &Header {
        unsafe { &*(self.0.get() as *const Header) }
    }

    fn is_header_connected(&self) -> bool {
        (self.header().status & STATUS_CONNECTED_FLAG) != 0
    }

    fn raw_var_headers(&self) -> &[VarHeaderRaw] {
        let header = self.header();
        unsafe {
            from_raw_parts(
                (self.0.get() as *const u8).offset(header.var_header_offset as isize)
                    as *const VarHeaderRaw,
                header.num_vars as usize,
            )
        }
    }

    fn get_var_headers(&self) -> VarHeaders {
        self.raw_var_headers()
            .iter()
            .filter_map(|var_header_raw| {
                let var_header = VarHeader::from_raw(var_header_raw).ok()?;
                Some((var_header.name.clone(), var_header))
            })
            .collect()
    }

    fn data(&self, header: &Header, buffer: &VarBuf) -> &[u8] {
        unsafe {
            from_raw_parts(
                (self.0.get() as *const u8).offset(buffer.buf_offset as isize),
                header.buf_len as usize,
            )
        }
    }

    fn raw_session_info(&self) -> &[u8] {
        let header = self.header();
        unsafe {
            from_raw_parts(
                (self.0.get() as *const u8).offset(header.session_info_offset as isize),
                header.session_info_len as usize,
            )
        }
    }
}

struct DataValidEvent {
    handle: windows_util::SafeHandle,
}

impl DataValidEvent {
    async fn connect() -> Self {
        let poll_delay = Duration::from_millis(250);
        loop {
            {
                let handle_opt = unsafe {
                    OpenEventA(
                        SYNCHRONIZATION_SYNCHRONIZE,
                        false,
                        PCSTR::from_raw(DATAVALIDEVENTNAME.as_ptr()),
                    )
                }
                .ok()
                .and_then(windows_util::SafeHandle::new);
                if let Some(handle) = handle_opt {
                    return Self { handle };
                }
            }
            tokio::time::sleep(poll_delay).await;
        }
    }

    async fn wait(&self, timeout: Option<Duration>) {
        let handle = unsafe { self.handle.get() };
        let millis = timeout.map_or(INFINITE, |v| v.as_millis() as u32);
        spawn_blocking(move || unsafe {
            WaitForSingleObject(handle, millis);
        })
        .await
        .ok();
    }
}
