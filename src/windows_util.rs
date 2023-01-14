use std::ffi::c_void;
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
}
