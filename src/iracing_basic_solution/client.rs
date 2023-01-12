use super::constants::{CLIENT_TIMEOUT, DATAVALIDEVENTNAME, MEMMAPFILENAME, STATUS_CONNECTED};
use super::header::{Header, VarHeader, VarHeaderRaw};
use super::string_decoding::cp1252_to_string;
use super::DataPoint;
use std::collections::HashMap;
use std::ffi::CString;
use std::slice::from_raw_parts;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use winapi::shared::minwindef::{FALSE, LPVOID};
use winapi::um::handleapi::CloseHandle;
use winapi::um::memoryapi::{MapViewOfFile, UnmapViewOfFile, FILE_MAP_READ};
use winapi::um::synchapi::{OpenEventA, WaitForSingleObject};
use winapi::um::winbase::OpenFileMappingA;
use winapi::um::winnt::{HANDLE, SYNCHRONIZE};
use yaml_rust::{Yaml, YamlLoader};

#[derive(Debug)]
pub struct Client {
    initialized: bool,
    data: Option<(Header, Vec<u8>)>,
    vars: HashMap<String, VarHeader>,
    last_tick_count: i32,
    last_valid_time: SystemTime,
    shared_mem: Option<SafeFileView>,
    mem_map_file: Option<SafeHandle>,
    data_valid_event: Option<SafeHandle>,
}

impl Drop for Client {
    fn drop(&mut self) {
        self.disconnect();
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    pub fn new() -> Self {
        Self {
            initialized: false,
            data: None,
            vars: HashMap::new(),
            last_tick_count: std::i32::MAX,
            last_valid_time: SystemTime::UNIX_EPOCH,
            shared_mem: None,
            mem_map_file: None,
            data_valid_event: None,
        }
    }

    pub fn disconnect(&mut self) {
        self.initialized = false;
        self.shared_mem = None;
        self.mem_map_file = None;
        self.data_valid_event = None;
    }

    pub fn connected(&self) -> bool {
        if !self.initialized {
            return false;
        }

        let shared_mem = self
            .shared_mem
            .as_ref()
            .expect("Shared mem init is guaranteed through the check above");

        let header = unsafe { &*(shared_mem.inner as *const Header) };
        if header.status & STATUS_CONNECTED == 0 {
            return false;
        }
        let elapsed = match SystemTime::now().duration_since(self.last_valid_time) {
            Ok(v) => v,
            Err(_) => return false,
        };
        elapsed < CLIENT_TIMEOUT
    }

    pub fn wait_for_data(&mut self, timeout: Duration) -> bool {
        if !self.startup_if_uninit() {
            sleep(timeout);
            return false;
        }

        if self.get_new_data() {
            return true;
        }

        if let Some(ref handle) = self.data_valid_event {
            unsafe {
                WaitForSingleObject(handle.inner, timeout.as_millis() as u32);
            }
        }

        self.get_new_data()
    }

    fn invalidate_next_package(&mut self) {
        self.last_tick_count = std::i32::MAX;
    }

    fn startup_if_uninit(&mut self) -> bool {
        self.initialized || self.startup()
    }

    pub fn startup(&mut self) -> bool {
        self.initialized = self.startup_impl();
        self.initialized
    }

    fn startup_impl(&mut self) -> bool {
        if self.mem_map_file.is_none() {
            self.mem_map_file = SafeHandle::new(unsafe {
                let memmap_file_name = CString::new(MEMMAPFILENAME).unwrap();
                OpenFileMappingA(FILE_MAP_READ, FALSE, memmap_file_name.as_ptr())
            });
            self.invalidate_next_package();
        }

        let mmf = if let Some(ref mmf) = self.mem_map_file {
            mmf
        } else {
            return false;
        };

        if self.shared_mem.is_none() {
            self.shared_mem =
                SafeFileView::new(unsafe { MapViewOfFile(mmf.inner, FILE_MAP_READ, 0, 0, 0) });
            self.invalidate_next_package();
        }

        if self.shared_mem.is_none() {
            return false;
        }

        if self.data_valid_event.is_none() {
            self.data_valid_event = SafeHandle::new(unsafe {
                let data_valid_event_name = CString::new(DATAVALIDEVENTNAME).unwrap();
                OpenEventA(SYNCHRONIZE, FALSE, data_valid_event_name.as_ptr())
            });
            self.invalidate_next_package();
        }

        if self.data_valid_event.is_none() {
            return false;
        }

        true
    }

    fn get_new_data(&mut self) -> bool {
        if !self.startup_if_uninit() {
            self.data = None;
            self.invalidate_next_package();
            return false;
        }

        let shared_mem = self
            .shared_mem
            .as_ref()
            .expect("Shared mem init is guaranteed through the check above");

        let header = unsafe { &*(shared_mem.inner as *const Header) };

        let buffer_size_changed = match self.data {
            None => true,
            Some((ref h, _)) => h.buf_len != header.buf_len,
        };

        if buffer_size_changed {
            let var_headers = unsafe {
                from_raw_parts(
                    (shared_mem.inner as *const u8).offset(header.var_header_offset as isize)
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

        if header.status & STATUS_CONNECTED == 0 {
            self.data = None;
            self.invalidate_next_package();
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
                    (shared_mem.inner as *const u8).offset(buffer.buf_offset as isize),
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
        if !self.connected() {
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
        if !self.initialized {
            return None;
        }
        let header = &self.data.as_ref()?.0;
        let shared_mem = self.shared_mem.as_ref()?;
        let data = unsafe {
            from_raw_parts(
                (shared_mem.inner as *const u8).offset(header.session_info_offset as isize),
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

#[derive(Debug)]
struct SafeHandle {
    pub inner: HANDLE,
}

impl SafeHandle {
    fn new(inner: HANDLE) -> Option<Self> {
        if inner.is_null() {
            None
        } else {
            Some(Self { inner })
        }
    }
}

impl Drop for SafeHandle {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.inner);
        }
    }
}

#[derive(Debug)]
struct SafeFileView {
    pub inner: LPVOID,
}

impl SafeFileView {
    fn new(inner: LPVOID) -> Option<Self> {
        if inner.is_null() {
            None
        } else {
            Some(Self { inner })
        }
    }
}

impl Drop for SafeFileView {
    fn drop(&mut self) {
        unsafe {
            UnmapViewOfFile(self.inner);
        }
    }
}
