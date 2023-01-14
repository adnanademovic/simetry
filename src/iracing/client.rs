use super::util::{SafeFileView, SafeHandle};
use crate::iracing::string_decoding::cp1252_to_string;
use crate::iracing_basic_solution::header::{Header, VarHeader, VarHeaderRaw};
use crate::iracing_basic_solution::DataPoint;
use std::collections::HashMap;
use std::slice::from_raw_parts;
use std::time::{Duration, SystemTime};
use windows::core::PCSTR;
use windows::Win32::System::Memory::{MapViewOfFile, OpenFileMappingA, FILE_MAP_READ};
use windows::Win32::System::Threading::{
    OpenEventA, WaitForSingleObject, SYNCHRONIZATION_SYNCHRONIZE,
};
use yaml_rust::{Yaml, YamlLoader};

static DATAVALIDEVENTNAME: &[u8] = b"Local\\IRSDKDataValidEvent";
static MEMMAPFILENAME: &[u8] = b"Local\\IRSDKMemMapFileName";

const STATUS_CONNECTED_FLAG: i32 = 1;

const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);

pub struct Client {
    data: Option<(Header, Vec<u8>)>,
    vars: HashMap<String, VarHeader>,
    last_tick_count: i32,
    last_valid_time: SystemTime,

    _mem_map_file: SafeHandle,
    shared_mem: SafeFileView,
    data_valid_event: SafeHandle,
}

impl Client {
    pub async fn connect() -> Self {
        let poll_delay = Duration::from_millis(250);

        let mem_map_file;
        loop {
            match unsafe {
                OpenFileMappingA(
                    FILE_MAP_READ.0,
                    false,
                    PCSTR::from_raw(MEMMAPFILENAME.as_ptr()),
                )
            }
            .ok()
            .and_then(SafeHandle::new)
            {
                Some(val) => {
                    mem_map_file = val;
                    break;
                }
                None => tokio::time::sleep(poll_delay).await,
            }
        }

        let shared_mem;
        loop {
            match SafeFileView::new(unsafe {
                MapViewOfFile(mem_map_file.get(), FILE_MAP_READ, 0, 0, 0)
            }) {
                Some(val) => {
                    shared_mem = val;
                    break;
                }
                None => tokio::time::sleep(poll_delay).await,
            }
        }

        let data_valid_event;
        loop {
            match unsafe {
                OpenEventA(
                    SYNCHRONIZATION_SYNCHRONIZE,
                    false,
                    PCSTR::from_raw(DATAVALIDEVENTNAME.as_ptr()),
                )
            }
            .ok()
            .and_then(SafeHandle::new)
            {
                Some(val) => {
                    data_valid_event = val;
                    break;
                }
                None => tokio::time::sleep(poll_delay).await,
            }
        }

        Self {
            data: None,
            vars: HashMap::new(),
            last_tick_count: i32::MAX,
            last_valid_time: SystemTime::UNIX_EPOCH,
            _mem_map_file: mem_map_file,
            shared_mem,
            data_valid_event,
        }
    }

    pub fn is_connected(&self) -> bool {
        let header = unsafe { &*(self.shared_mem.get() as *const Header) };
        if header.status & STATUS_CONNECTED_FLAG == 0 {
            return false;
        }
        let elapsed = match SystemTime::now().duration_since(self.last_valid_time) {
            Ok(v) => v,
            Err(_) => return false,
        };
        elapsed < CLIENT_TIMEOUT
    }

    pub fn wait_for_data(&mut self, timeout: Duration) -> bool {
        if self.get_new_data() {
            return true;
        }

        unsafe {
            WaitForSingleObject(self.data_valid_event.get(), timeout.as_millis() as u32);
        }

        self.get_new_data()
    }

    fn get_new_data(&mut self) -> bool {
        let header = unsafe { &*(self.shared_mem.get() as *const Header) };

        let buffer_size_changed = match self.data {
            None => true,
            Some((ref h, _)) => h.buf_len != header.buf_len,
        };

        if buffer_size_changed {
            let var_headers = unsafe {
                from_raw_parts(
                    (self.shared_mem.get() as *const u8).offset(header.var_header_offset as isize)
                        as *const VarHeaderRaw,
                    header.num_vars as usize,
                )
            };
            self.vars = var_headers
                .iter()
                .filter_map(|var_header_raw| {
                    let var_header = VarHeader::from_raw(var_header_raw)?;
                    Some((var_header.name.clone(), var_header))
                })
                .collect()
        }

        if header.status & STATUS_CONNECTED_FLAG == 0 {
            self.data = None;
            self.last_tick_count = i32::MAX;
            return false;
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
            return false;
        }

        // Two attempts to retrieve data
        for _ in 0..2 {
            let tick_count = buffer.tick_count;
            let data = unsafe {
                from_raw_parts(
                    (self.shared_mem.get() as *const u8).offset(buffer.buf_offset as isize),
                    header.buf_len as usize,
                )
            };
            if tick_count == buffer.tick_count {
                self.last_tick_count = tick_count;
                self.last_valid_time = SystemTime::now();
                self.data = Some((header.clone(), data.to_vec()));
                return true;
            }
        }

        false
    }

    pub fn data(&self) -> Option<DataPoint> {
        if !self.is_connected() {
            return None;
        }
        let vars = &self.vars;
        let (header, raw_data) = self.data.as_ref()?;
        Some(DataPoint {
            vars,
            header,
            raw_data,
        })
    }

    pub fn session_info_raw(&self) -> Option<String> {
        let header = &self.data.as_ref()?.0;
        let data = unsafe {
            from_raw_parts(
                (self.shared_mem.get() as *const u8).offset(header.session_info_offset as isize),
                header.session_info_len as usize,
            )
        };
        cp1252_to_string(data).ok()
    }

    pub fn session_info(&self) -> Option<Yaml> {
        let data_string = self.session_info_raw()?;
        let mut items = YamlLoader::load_from_str(&data_string).ok()?;
        if items.is_empty() {
            return None;
        }
        Some(items.swap_remove(0))
    }
}
