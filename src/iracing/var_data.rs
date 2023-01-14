use crate::iracing::header::{VarHeader, VarType};
use byteorder::{LittleEndian, ReadBytesExt};

pub trait VarData: Sized {
    fn parse_from_raw(entry: usize, header: &VarHeader, data: &[u8]) -> Option<Self> {
        if entry >= header.count {
            return None;
        }
        let leftover = header.count - entry;
        let entry_size = header.var_type.byte_count();
        let start = header.offset + entry * entry_size;
        let end = start + leftover * entry_size;
        Self::parse(header.var_type, &data[start..end])
    }

    /// Read value from location in data.
    ///
    /// Count is guaranteed to be at least 1.
    fn parse(var_type: VarType, data: &[u8]) -> Option<Self>;
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Char(u8),
    Bool(bool),
    Int(i32),
    BitField(u32),
    Float(f32),
    Double(f64),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Char(v) => write!(f, "{}", v),
            Value::Bool(v) => write!(f, "{}", v),
            Value::Int(v) => write!(f, "{}", v),
            Value::BitField(v) => write!(f, "{:#032b}", v),
            Value::Float(v) => write!(f, "{}", v),
            Value::Double(v) => write!(f, "{}", v),
        }
    }
}

impl VarData for Value {
    fn parse(var_type: VarType, data: &[u8]) -> Option<Self> {
        match var_type {
            VarType::Char => VarData::parse(var_type, data).map(Value::Char),
            VarType::Bool => VarData::parse(var_type, data).map(Value::Bool),
            VarType::Int => VarData::parse(var_type, data).map(Value::Int),
            VarType::BitField => VarData::parse(var_type, data).map(Value::BitField),
            VarType::Float => VarData::parse(var_type, data).map(Value::Float),
            VarType::Double => VarData::parse(var_type, data).map(Value::Double),
        }
    }
}

impl<T: VarData> VarData for Vec<T> {
    fn parse(var_type: VarType, data: &[u8]) -> Option<Self> {
        Some(
            data.chunks(var_type.byte_count())
                .filter_map(|chunk| T::parse(var_type, chunk))
                .collect(),
        )
    }
}

impl VarData for u8 {
    fn parse(var_type: VarType, mut data: &[u8]) -> Option<Self> {
        match var_type {
            VarType::Char | VarType::Bool => data.read_u8().ok(),
            _ => None,
        }
    }
}

impl VarData for bool {
    fn parse(var_type: VarType, mut data: &[u8]) -> Option<Self> {
        match var_type {
            VarType::Char | VarType::Bool => data.read_u8().ok().map(|v| v != 0),
            _ => None,
        }
    }
}

impl VarData for i32 {
    fn parse(var_type: VarType, mut data: &[u8]) -> Option<Self> {
        match var_type {
            VarType::Char => data.read_u8().ok().map(|v| v as i32),
            VarType::Int => data.read_i32::<LittleEndian>().ok(),
            _ => None,
        }
    }
}

impl VarData for u32 {
    fn parse(var_type: VarType, mut data: &[u8]) -> Option<Self> {
        match var_type {
            VarType::BitField => data.read_u32::<LittleEndian>().ok(),
            _ => None,
        }
    }
}

impl VarData for f32 {
    fn parse(var_type: VarType, mut data: &[u8]) -> Option<Self> {
        match var_type {
            VarType::Float => data.read_f32::<LittleEndian>().ok(),
            _ => None,
        }
    }
}

impl VarData for f64 {
    fn parse(var_type: VarType, mut data: &[u8]) -> Option<Self> {
        match var_type {
            VarType::Float => data.read_f32::<LittleEndian>().ok().map(|v| v as f64),
            VarType::Double => data.read_f64::<LittleEndian>().ok(),
            _ => None,
        }
    }
}
