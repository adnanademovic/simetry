use std::ffi::c_void;
use std::string::FromUtf16Error;
use std::time::Duration;
use windows::core::PCSTR;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Memory::{
    MapViewOfFile, OpenFileMappingA, UnmapViewOfFile, FILE_MAP_READ,
};

#[derive(Debug)]
pub struct SafeHandle {
    inner: HANDLE,
}

impl SafeHandle {
    pub fn new(inner: HANDLE) -> Option<Self> {
        if inner.is_invalid() {
            None
        } else {
            Some(Self { inner })
        }
    }

    pub unsafe fn get(&self) -> HANDLE {
        self.inner
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
pub struct SafeFileView {
    pub inner: *const c_void,
}

unsafe impl Send for SafeFileView {}

impl SafeFileView {
    pub fn new(inner: *const c_void) -> Option<Self> {
        if inner.is_null() {
            None
        } else {
            Some(Self { inner })
        }
    }

    pub unsafe fn get(&self) -> *const c_void {
        self.inner
    }
}

impl Drop for SafeFileView {
    fn drop(&mut self) {
        unsafe {
            UnmapViewOfFile(self.inner);
        }
    }
}

pub struct SharedMemory {
    _handle: SafeHandle,
    file_view: SafeFileView,
}

impl SharedMemory {
    pub async fn connect(name: &[u8], poll_delay: Duration) -> Self {
        let handle;
        loop {
            match unsafe {
                OpenFileMappingA(FILE_MAP_READ.0, false, PCSTR::from_raw(name.as_ptr()))
            }
            .ok()
            .and_then(SafeHandle::new)
            {
                Some(val) => {
                    handle = val;
                    break;
                }
                None => {
                    tokio::time::sleep(poll_delay).await;
                }
            }
        }

        let file_view;
        loop {
            match SafeFileView::new(unsafe { MapViewOfFile(handle.get(), FILE_MAP_READ, 0, 0, 0) })
            {
                Some(val) => {
                    file_view = val;
                    break;
                }
                None => tokio::time::sleep(poll_delay).await,
            }
        }

        Self {
            _handle: handle,
            file_view,
        }
    }

    pub unsafe fn get(&self) -> *const c_void {
        self.file_view.get()
    }

    pub unsafe fn get_as<T>(&self) -> &T {
        &(*(self.get() as *const T))
    }

    pub unsafe fn copy_as<T: Copy>(&self) -> T {
        *(self.get() as *const T)
    }
}

/// Decoding table based on data from https://www.cp1252.com and ftp://ftp.unicode.org/Public/MAPPINGS/VENDORS/MICSFT/WINDOWS/CP1252.TXT

pub fn cp1252_to_string(data: &[u8]) -> Result<String, FromUtf16Error> {
    let mut codepoints = Vec::with_capacity(data.len());
    for byte in data {
        if *byte == 0 {
            break;
        }
        codepoints.push(MAPPING[*byte as usize]);
    }
    String::from_utf16(&codepoints)
}

const MAPPING: [u16; 256] = [
    0x0000, 0x0001, 0x0002, 0x0003, 0x0004, 0x0005, 0x0006, 0x0007, 0x0008, 0x0009, 0x000A, 0x000B,
    0x000C, 0x000D, 0x000E, 0x000F, 0x0010, 0x0011, 0x0012, 0x0013, 0x0014, 0x0015, 0x0016, 0x0017,
    0x0018, 0x0019, 0x001A, 0x001B, 0x001C, 0x001D, 0x001E, 0x001F, 0x0020, 0x0021, 0x0022, 0x0023,
    0x0024, 0x0025, 0x0026, 0x0027, 0x0028, 0x0029, 0x002A, 0x002B, 0x002C, 0x002D, 0x002E, 0x002F,
    0x0030, 0x0031, 0x0032, 0x0033, 0x0034, 0x0035, 0x0036, 0x0037, 0x0038, 0x0039, 0x003A, 0x003B,
    0x003C, 0x003D, 0x003E, 0x003F, 0x0040, 0x0041, 0x0042, 0x0043, 0x0044, 0x0045, 0x0046, 0x0047,
    0x0048, 0x0049, 0x004A, 0x004B, 0x004C, 0x004D, 0x004E, 0x004F, 0x0050, 0x0051, 0x0052, 0x0053,
    0x0054, 0x0055, 0x0056, 0x0057, 0x0058, 0x0059, 0x005A, 0x005B, 0x005C, 0x005D, 0x005E, 0x005F,
    0x0060, 0x0061, 0x0062, 0x0063, 0x0064, 0x0065, 0x0066, 0x0067, 0x0068, 0x0069, 0x006A, 0x006B,
    0x006C, 0x006D, 0x006E, 0x006F, 0x0070, 0x0071, 0x0072, 0x0073, 0x0074, 0x0075, 0x0076, 0x0077,
    0x0078, 0x0079, 0x007A, 0x007B, 0x007C, 0x007D, 0x007E, 0x007F, 0x20AC, 0x0020, 0x201A, 0x0192,
    0x201E, 0x2026, 0x2020, 0x2021, 0x02C6, 0x2030, 0x0160, 0x2039, 0x0152, 0x0020, 0x017D, 0x0020,
    0x0020, 0x2018, 0x2019, 0x201C, 0x201D, 0x2022, 0x2013, 0x2014, 0x02DC, 0x2122, 0x0161, 0x203A,
    0x0153, 0x0020, 0x017E, 0x0178, 0x00A0, 0x00A1, 0x00A2, 0x00A3, 0x00A4, 0x00A5, 0x00A6, 0x00A7,
    0x00A8, 0x00A9, 0x00AA, 0x00AB, 0x00AC, 0x00AD, 0x00AE, 0x00AF, 0x00B0, 0x00B1, 0x00B2, 0x00B3,
    0x00B4, 0x00B5, 0x00B6, 0x00B7, 0x00B8, 0x00B9, 0x00BA, 0x00BB, 0x00BC, 0x00BD, 0x00BE, 0x00BF,
    0x00C0, 0x00C1, 0x00C2, 0x00C3, 0x00C4, 0x00C5, 0x00C6, 0x00C7, 0x00C8, 0x00C9, 0x00CA, 0x00CB,
    0x00CC, 0x00CD, 0x00CE, 0x00CF, 0x00D0, 0x00D1, 0x00D2, 0x00D3, 0x00D4, 0x00D5, 0x00D6, 0x00D7,
    0x00D8, 0x00D9, 0x00DA, 0x00DB, 0x00DC, 0x00DD, 0x00DE, 0x00DF, 0x00E0, 0x00E1, 0x00E2, 0x00E3,
    0x00E4, 0x00E5, 0x00E6, 0x00E7, 0x00E8, 0x00E9, 0x00EA, 0x00EB, 0x00EC, 0x00ED, 0x00EE, 0x00EF,
    0x00F0, 0x00F1, 0x00F2, 0x00F3, 0x00F4, 0x00F5, 0x00F6, 0x00F7, 0x00F8, 0x00F9, 0x00FA, 0x00FB,
    0x00FC, 0x00FD, 0x00FE, 0x00FF,
];
