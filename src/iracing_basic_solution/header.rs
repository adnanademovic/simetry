use super::constants::{MAX_DESC, MAX_STRING};
use crate::iracing::string_decoding::cp1252_to_string;
use std::collections::HashMap;

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
    pub(crate) fn from_raw(raw: &VarHeaderRaw) -> Option<Self> {
        Some(Self {
            var_type: VarType::from_raw(raw.var_type)?,
            offset: raw.offset as usize,
            count: raw.count as usize,
            count_as_time: raw.count_as_time != 0,
            name: cp1252_to_string(&raw.name).ok()?,
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
    fn from_raw(raw: i32) -> Option<VarType> {
        Some(match raw {
            0 => VarType::Char,
            1 => VarType::Bool,
            2 => VarType::Int,
            3 => VarType::BitField,
            4 => VarType::Float,
            5 => VarType::Double,
            _ => return None,
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
