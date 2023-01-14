use std::ffi::c_void;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Memory::UnmapViewOfFile;

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

    pub fn get(&self) -> HANDLE {
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

    pub fn get(&self) -> *const c_void {
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
