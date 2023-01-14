use super::header::{DiskSubHeader, Header, VarHeader, VarHeaderRaw};
use super::DataPoint;
use crate::iracing::string_decoding::cp1252_to_string;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use std::{io, slice};
use yaml_rust::{Yaml, YamlLoader};

#[derive(Debug)]
pub struct DiskClient {
    file: File,
    header: Header,
    vars: HashMap<String, VarHeader>,
    raw_data: Vec<u8>,
    session_info: Yaml,
}

impl DiskClient {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let header: Header = read_struct(&mut file)?;
        let _disk_sub_header: DiskSubHeader = read_struct(&mut file)?;

        let mut session_info_buffer = vec![0u8; header.session_info_len as usize];
        file.seek(SeekFrom::Start(header.session_info_offset as u64))?;
        file.read_exact(&mut session_info_buffer)?;

        let session_info_str = match cp1252_to_string(&session_info_buffer) {
            Ok(v) => v,
            Err(err) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Could not decode CP1252 data: {}", err),
                ));
            }
        };

        let mut session_info_array = match YamlLoader::load_from_str(&session_info_str) {
            Ok(v) => v,
            Err(err) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Could not read YAML data: {}", err),
                ))
            }
        };
        if session_info_array.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Session info does not have any data",
            ));
        }
        let session_info = session_info_array.swap_remove(0);

        file.seek(SeekFrom::Start(header.var_header_offset as u64))?;
        let vars = (0..header.num_vars)
            .filter_map(|_| {
                let raw: VarHeaderRaw = read_struct(&mut file).ok()?;
                let header = VarHeader::from_raw(&raw)?;
                Some((header.name.clone(), header))
            })
            .collect();
        let raw_data = vec![0u8; header.buf_len as usize];
        file.seek(SeekFrom::Start(header.var_buf[0].buf_offset as u64))?;

        Ok(Self {
            file,
            header,
            vars,
            raw_data,
            session_info,
        })
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn vars(&self) -> &HashMap<String, VarHeader> {
        &self.vars
    }

    pub fn session_info(&self) -> &Yaml {
        &self.session_info
    }

    pub fn next_data(&mut self) -> io::Result<DataPoint> {
        self.file.read_exact(&mut self.raw_data)?;
        Ok(DataPoint {
            vars: &self.vars,
            header: &self.header,
            raw_data: &self.raw_data,
        })
    }
}

fn read_struct<T, R: Read>(mut read: R) -> io::Result<T> {
    let num_bytes = ::std::mem::size_of::<T>();
    unsafe {
        let mut s = ::std::mem::zeroed();
        let buffer = slice::from_raw_parts_mut(&mut s as *mut T as *mut u8, num_bytes);
        match read.read_exact(buffer) {
            Ok(()) => Ok(s),
            Err(e) => {
                ::std::mem::forget(s);
                Err(e)
            }
        }
    }
}
