use crate::iracing::constants::IRSDK_VER;
use crate::iracing::header::VarHeaderRaw;
use crate::iracing::session_info::parse_session_info;
use crate::iracing::{DiskSubHeader, Header, SimState, VarHeader, VarHeaders};
use anyhow::{bail, Result};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use std::sync::Arc;
use yaml_rust::Yaml;

#[derive(Debug)]
pub struct DiskClient {
    file: File,
    header: Arc<Header>,
    sub_header: DiskSubHeader,
    variables: Arc<VarHeaders>,
    session_info: Arc<Yaml>,
}

impl DiskClient {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = File::open(path)?;
        let header: Arc<Header> = Arc::new(read_struct(&mut file)?);

        let sdk_version = header.ver;
        if sdk_version != IRSDK_VER {
            bail!("iRacing SDK version mismatch: expected {IRSDK_VER}, received {sdk_version}");
        }

        let sub_header: DiskSubHeader = read_struct(&mut file)?;

        let mut session_info_buffer = vec![0u8; header.session_info_len as usize];
        file.seek(SeekFrom::Start(header.session_info_offset as u64))?;
        file.read_exact(&mut session_info_buffer)?;

        let session_info = Arc::new(parse_session_info(&session_info_buffer)?);

        file.seek(SeekFrom::Start(header.var_header_offset as u64))?;
        let variables = Arc::new(
            (0..header.num_vars)
                .filter_map(|_| {
                    let raw: VarHeaderRaw = read_struct(&mut file).ok()?;
                    let header = VarHeader::from_raw(&raw).ok()?;
                    Some((header.name.clone(), header))
                })
                .collect(),
        );
        file.seek(SeekFrom::Start(header.var_buf[0].buf_offset as u64))?;

        Ok(Self {
            file,
            header,
            sub_header,
            variables,
            session_info,
        })
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn sub_header(&self) -> &DiskSubHeader {
        &self.sub_header
    }

    pub fn variables(&self) -> &VarHeaders {
        &self.variables
    }

    pub fn session_info(&self) -> &Yaml {
        &self.session_info
    }

    pub fn next_sim_state(&mut self) -> Option<SimState> {
        let mut raw_data = vec![0u8; self.header.buf_len as usize];
        self.file.read_exact(&mut raw_data).ok()?;
        Some(SimState::new(
            self.header.clone(),
            Arc::clone(&self.variables),
            raw_data,
            Arc::clone(&self.session_info),
        ))
    }
}

fn read_struct<T, R: Read>(mut read: R) -> Result<T> {
    let num_bytes = std::mem::size_of::<T>();
    unsafe {
        let mut s = std::mem::zeroed();
        let buffer = std::slice::from_raw_parts_mut(&mut s as *mut T as *mut u8, num_bytes);
        match read.read_exact(buffer) {
            Ok(()) => Ok(s),
            Err(e) => {
                std::mem::forget(s);
                Err(e)?
            }
        }
    }
}
