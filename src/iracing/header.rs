use crate::windows_util::cp1252_to_string;
use anyhow::{bail, Result};
use std::collections::HashMap;

const MAX_BUFS: usize = 4;

pub(super) const MAX_STRING: usize = 32;
// descriptions can be longer than max_string!
pub(super) const MAX_DESC: usize = 64;

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct Header {
    /// this api header version, see IRSDK_VER
    pub ver: i32,
    /// bitfield using irsdk_StatusField
    pub status: i32,
    /// ticks per second (60 or 360 etc)
    pub tick_rate: i32,

    // session information, updated periodically
    /// Incremented when session info changes
    pub session_info_update: i32,
    /// Length in bytes of session info string
    pub session_info_len: i32,
    /// Session info, encoded in YAML format
    pub session_info_offset: i32,

    // State data, output at tick_rate
    /// Length of array pointed to by var_header_offset
    pub num_vars: i32,
    /// Offset to irsdk_varHeader[num_vars] array, Describes the variables received in var_buf
    pub var_header_offset: i32,

    /// Less or equal to IRSDK_MAX_BUFS (3 for now)
    pub num_buf: i32,
    /// Length in bytes for one line
    pub buf_len: i32,
    /// (16 byte align)
    pub pad: [i32; 2],
    /// Buffers of data being written to
    pub var_buf: [VarBuf; MAX_BUFS],
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct VarBuf {
    /// Used to detect changes in data
    pub tick_count: i32,
    /// Offset from header
    pub buf_offset: i32,
    /// (16 byte align)
    pub pad: [i32; 2],
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct DiskSubHeader {
    pub session_start_date: i64,
    pub session_start_time: f64,
    pub session_end_time: f64,
    pub session_lap_count: i32,
    pub session_record_count: i32,
}

#[repr(C)]
pub(crate) struct VarHeaderRaw {
    var_type: i32,
    offset: i32,
    count: i32,
    count_as_time: u8,
    pad: [u8; 3],
    name: [u8; MAX_STRING],
    desc: [u8; MAX_DESC],
    unit: [u8; MAX_STRING],
}

#[derive(Debug)]
pub struct VarHeader {
    /// VarType
    pub var_type: VarType,
    /// Offset from start of buffer row
    pub offset: usize,
    /// Number of entries for arrays
    ///
    /// So length in bytes would be VarTypeBytes[var_type] * count
    pub count: usize,

    pub count_as_time: bool,

    pub name: String,
    pub desc: String,
    /// something like "kg/m^2"
    pub unit: String,
}

pub type VarHeaders = HashMap<String, VarHeader>;

impl VarHeader {
    pub(crate) fn from_raw(raw: &VarHeaderRaw) -> Result<Self> {
        Ok(Self {
            var_type: VarType::from_raw(raw.var_type)?,
            offset: raw.offset as usize,
            count: raw.count as usize,
            count_as_time: raw.count_as_time != 0,
            name: cp1252_to_string(&raw.name)?,
            desc: cp1252_to_string(&raw.desc).unwrap_or_default(),
            unit: cp1252_to_string(&raw.unit).unwrap_or_default(),
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub enum VarType {
    // 1 byte
    Char = 0,
    Bool = 1,
    // 4 bytes
    Int = 2,
    BitField = 3,
    Float = 4,
    // 8 bytes
    Double = 5,
}

impl VarType {
    fn from_raw(raw: i32) -> Result<VarType> {
        Ok(match raw {
            0 => VarType::Char,
            1 => VarType::Bool,
            2 => VarType::Int,
            3 => VarType::BitField,
            4 => VarType::Float,
            5 => VarType::Double,
            _ => bail!("Invalid data type ID: {raw}"),
        })
    }

    pub fn byte_count(self) -> usize {
        match self {
            VarType::Char => 1,
            VarType::Bool => 1,
            VarType::Int => 4,
            VarType::BitField => 4,
            VarType::Float => 4,
            VarType::Double => 8,
        }
    }
}
